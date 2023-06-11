use sqlx::{PgPool, Row};
use crate::{
    routes::{
        register::RegisterForm,
        login::LoginForm,
        diet_nutrition::DietInfoNutrient,
        meals::{MealInfoFood, MealInfoNutrient},
        food_search::{SearchFoodServing, SearchFood}
    },
    utils::{
        hash::sha256str,
        time::calculate_age
    },
    models::*
};
use uuid::Uuid;
use anyhow::{Error, Result};
use chrono::{Utc, Datelike};

pub async fn create_user_account(data : &RegisterForm, dbpool : &PgPool) -> Result<(), sqlx::Error> {
    let password_hash = sha256str(data.password.as_str());

    let query_result = sqlx::query("INSERT INTO user_account(name, email, gender, weight, birthdate, password_hash) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(&data.name)
        .bind(&data.email)
        .bind(data.gender.to_string())
        .bind(data.weight)
        .bind(&data.birthdate)
        .bind(&password_hash)
        .fetch_one(dbpool)
        .await?;

    let new_user_id : i32 = query_result.try_get("id")?;

    sqlx::query("INSERT INTO credentials(user_id, password) VALUES ($1, $2)")
        .bind(new_user_id)
        .bind(&data.password)
        .execute(dbpool)
        .await
        .ok();

    Ok(())
}

pub async fn authenticate_user(data : &LoginForm, dbpool : &PgPool) -> Result<String> {
    let query_result = sqlx::query("SELECT id, password_hash FROM user_account WHERE email = $1")
        .bind(&data.email)
        .fetch_one(dbpool)
        .await?;

    let user_id : i32 = query_result.try_get("id")?;
    let password_hash : String = query_result.try_get("password_hash")?;

    let attempt_hash = sha256str(data.password.as_str());

    if attempt_hash != password_hash {
        return Err(Error::msg("Passwords don't match"));
    }

    let session_id = Uuid::new_v4();
    let expiry_date = Utc::now();
    let expiry_date = expiry_date.with_year(expiry_date.year() + 1);

    sqlx::query("INSERT INTO user_session(id, user_id, expiry_date) VALUES ($1, $2, $3)")
        .bind(session_id)
        .bind(user_id)
        .bind(expiry_date)
        .execute(dbpool)
        .await?;

    Ok(session_id.to_string())
}

pub async fn get_session_user_id(session_id : &Uuid, dbpool : &PgPool) -> Option<i32> {
    let query_result = sqlx::query("SELECT user_id FROM user_session WHERE id = $1")
        .bind(session_id)
        .fetch_one(dbpool)
        .await
        .ok()?;

    query_result.try_get::<i32, _>("user_id").ok()
}

pub async fn fetch_user_diets(user_id : i32, dbpool : &PgPool) -> Result<Vec<Diet>> {
    let diets = sqlx::query_as::<_, Diet>("SELECT * FROM diet WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(dbpool)
        .await?;

    Ok(diets)
}

pub async fn delete_session(session_id : Uuid, dbpool : &PgPool) -> Result<()> {
    sqlx::query("DELETE FROM user_session WHERE id = $1")
        .bind(session_id)
        .execute(dbpool)
        .await?;

    Ok(())
}

pub async fn get_diet_user_id(diet_id : i32, dbpool : &PgPool) -> Option<i32> {
    let query_result = sqlx::query("SELECT user_id FROM diet WHERE id = $1")
        .bind(diet_id)
        .fetch_one(dbpool)
        .await
        .ok()?;

    query_result.try_get::<i32, _>("user_id").ok()
}

/*
pub async fn fetch_diet_nutrition(diet_id : i32, dbpool : &PgPool) -> Option<Vec<DietNutrient>> {
    let diet_nutrients = sqlx::query_as::<_, DietNutrient>("SELECT * FROM diet_nutrition WHERE diet_id = $1")
        .bind(diet_id)
        .fetch_all(dbpool)
        .await
        .ok()?;

    Some(diet_nutrients)
}
*/

pub async fn fetch_diet_info_nutrition(diet_id : i32, dbpool : &PgPool) -> Option<Vec<DietInfoNutrient>> {
    let diet_info_nutrients = sqlx::query_as::<_, DietInfoNutrient>("SELECT nutrient.name AS name, diet_nutrition.min_intake AS min_amount, diet_nutrition.max_intake AS max_amount, nutrient.unit AS unit, diet_nutrition.relative AS relative FROM diet_nutrition JOIN nutrient ON nutrient.id = diet_nutrition.nutrient_id WHERE diet_nutrition.diet_id = $1")
        .bind(diet_id)
        .fetch_all(dbpool)
        .await
        .ok()?;

    Some(diet_info_nutrients)
}

pub async fn fetch_nutrients(dbpool : &PgPool) -> Option<Vec<Nutrient>> {
    let nutrients = sqlx::query_as::<_, Nutrient>("SELECT * FROM nutrient")
        .fetch_all(dbpool)
        .await
        .ok()?;

    Some(nutrients)
}

pub async fn fetch_diet_meals(diet_id : i32, dbpool : &PgPool) -> Option<Vec<Meal>> {
    let meals = sqlx::query_as::<_, Meal>("SELECT * FROM meal WHERE diet_id = $1")
        .bind(diet_id)
        .fetch_all(dbpool)
        .await
        .ok()?;

    Some(meals)
}

/* TODO: Clean up this function (taken from old codebase, needs refactoring) */
pub async fn fetch_meal_info_foods(meal_id : i32, dbpool : &PgPool) -> Option<Vec<MealInfoFood>> {
	let query_foods = async {
		sqlx::query("SELECT food.id AS id, food.name AS name, meal_serving.id AS meal_serving_id, serving.id AS serving_id, serving.amount AS serving_base, meal_serving.amount AS amount, serving.unit AS unit, serving.relative AS relative FROM meal_serving JOIN serving ON meal_serving.serving_id = serving.id JOIN food ON serving.food_id = food.id WHERE meal_serving.meal_id = $1")
			.bind(meal_id)
			.fetch_all(dbpool)
			.await
	};

	let foods = match query_foods.await {
		Ok(r) => r,
		Err(_) => return None
	};

	let mut foods_info : Vec<MealInfoFood> = vec![];
	for food in foods {
		let food_id : i32 = food.try_get("id").ok()?;
		let food_name : String = food.try_get("name").ok()?;
		let meal_serving_id : i32 = food.try_get("meal_serving_id").ok()?;
		let serving_id : i32 = food.try_get("serving_id").ok()?;
		let mut serving_base : f64 = food.try_get("serving_base").ok()?;
		let serving_amount : f64 = food.try_get("amount").ok()?;
		let serving_unit : String = food.try_get("unit").ok()?;
		let serving_relative : Option<i32> = food.try_get("relative").ok()?;
		let mut serving_rel_amount : f64 = 0.0;

		if let Some(_) = serving_relative {
			serving_base = 1.0;

			let row = match sqlx::query("SELECT amount FROM serving WHERE serving.id = $1")
				.bind(serving_id)
				.fetch_one(dbpool)
				.await {
				Ok(r) => r,
				Err(_) => continue
			};

			serving_rel_amount = row.try_get("amount").unwrap();
		}

		let query_nutrients = async {
			if let Some(id) = serving_relative {
				sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit, serving.amount AS serving_base_amount FROM serving_nutrient JOIN serving ON serving.id = serving_nutrient.serving_id JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving.id = $1")
					.bind(id)
					.fetch_all(dbpool)
					.await
			} else {
				sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN serving ON serving.id = serving_nutrient.serving_id JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving.id = $1")
					.bind(serving_id)
					.fetch_all(dbpool)
					.await
			}
		};

		let nutrients = match query_nutrients.await {
			Ok(r) => r,
			Err(_) => continue
		};

		let mut base_nutrients : Vec<MealInfoNutrient> = vec![];
		for nutrient in nutrients {
			let nutrient_name : String = nutrient.try_get("name").ok()?;
			let mut nutrient_amount : f64 = nutrient.try_get("amount").ok()?;
			let nutrient_unit : String = nutrient.try_get("unit").ok()?;

			if let Some(_) = serving_relative {
				let serving_base_amount : f64 = nutrient.try_get("serving_base_amount").ok()?;
				nutrient_amount *= serving_rel_amount / serving_base_amount;
			}

			let nutrient_info = MealInfoNutrient {
				name: nutrient_name,
				amount: nutrient_amount,
				unit: nutrient_unit
			};
			base_nutrients.push(nutrient_info);
		}

		let food = MealInfoFood {
			id: food_id,
			name: food_name,
			meal_serving_id,
			serving_id,
			serving_base,
			serving_amount,
			serving_unit,
			base_nutrients
		};
		foods_info.push(food);
	}

    Some(foods_info)
}

pub async fn fetch_user_account(user_id : i32, dbpool : &PgPool) -> Option<UserAccount> {
    let user = sqlx::query_as::<_, UserAccount>("SELECT * FROM user_account WHERE id = $1")
        .bind(user_id)
        .fetch_one(dbpool)
        .await
        .ok()?;

    Some(user)
}

pub async fn create_diet(user_id : i32, diet_name : &String, dbpool : &PgPool) -> Option<()> {
    let diet_id = sqlx::query("INSERT INTO diet(name, user_id) VALUES($1, $2) RETURNING id")
        .bind(diet_name)
        .bind(user_id)
        .fetch_one(dbpool)
        .await
        .ok()?;
    let diet_id : i32 = diet_id.try_get("id").ok()?;

    let user_account = fetch_user_account(user_id, dbpool).await?;

    let user_age = calculate_age(&user_account.birthdate);

    // TODO: Check if 'query_as' fails if the SQL returns no rows
    let default_nutrition = sqlx::query_as::<_, DefaultNutrient>("SELECT * FROM default_nutrition WHERE gender = $1 AND $2 >= age_min AND ($3 < age_max OR age_max IS NULL)")
        .bind(user_account.gender)
        .bind(user_age)
        .bind(user_age)
        .fetch_all(dbpool)
        .await
        .ok()?;

    for nutrient in default_nutrition {
        sqlx::query("INSERT INTO diet_nutrition(diet_id, nutrient_id, min_intake, max_intake, relative) VALUES ($1, $2, $3, $4, $5)")
            .bind(diet_id)
            .bind(nutrient.nutrient_id)
            .bind(nutrient.min_intake)
            .bind(nutrient.max_intake)
            .bind(nutrient.relative)
            .execute(dbpool)
            .await
            .ok();
    }

    Some(())
}

pub async fn edit_diet(diet_id : i32, diet_name : &String, dbpool : &PgPool) -> Option<()> {
    sqlx::query("UPDATE diet SET name = $1 WHERE id = $2")
        .bind(diet_name)
        .bind(diet_id)
        .execute(dbpool)
        .await
        .ok()?;

    Some(())
}

pub async fn delete_diet(diet_id : i32, dbpool : &PgPool) -> Result<()> {
    sqlx::query("DELETE FROM meal_serving WHERE meal_id IN (SELECT meal_serving.meal_id FROM meal_serving JOIN meal ON meal.id = meal_serving.meal_id WHERE diet_id = $1)")
        .bind(diet_id)
        .execute(dbpool)
        .await?;

    sqlx::query("DELETE FROM meal WHERE diet_id = $1")
        .bind(diet_id)
        .execute(dbpool)
        .await?;

    sqlx::query("DELETE FROM diet_nutrition WHERE diet_id = $1")
        .bind(diet_id)
        .execute(dbpool)
        .await?;

    sqlx::query("DELETE FROM diet WHERE id = $1")
        .bind(diet_id)
        .execute(dbpool)
        .await?;

    Ok(())
}

pub async fn create_meal(diet_id : i32, meal_name : &String, dbpool : &PgPool) -> Option<i32> {
    let query_result = sqlx::query("INSERT INTO meal(diet_id, name) VALUES ($1, $2) RETURNING id")
        .bind(diet_id)
        .bind(meal_name)
        .fetch_one(dbpool)
        .await
        .ok()?;

    let meal_id : i32 = query_result.try_get("id").ok()?;

    Some(meal_id)
}

pub async fn get_meal_user_id(meal_id : i32, dbpool : &PgPool) -> Option<i32> {
    let query_result = sqlx::query("SELECT diet_id FROM meal WHERE id = $1")
        .bind(meal_id)
        .fetch_one(dbpool)
        .await
        .ok()?;

    let diet_id : i32 = query_result.try_get("diet_id").ok()?;

    get_diet_user_id(diet_id, dbpool).await
}

pub async fn delete_meal(meal_id : i32, dbpool : &PgPool) -> Result<()> {
    sqlx::query("DELETE FROM meal_serving WHERE meal_id = $1")
        .bind(meal_id)
        .execute(dbpool)
        .await?;

    sqlx::query("DELETE FROM meal WHERE id = $1")
        .bind(meal_id)
        .execute(dbpool)
        .await?;

    Ok(())
}

pub async fn search_foods(food_name : &String, dbpool : &PgPool) -> Option<Vec<SearchFood>> {
    // setup SQL search strings (for the ILIKE keyword)
    let best_search = format!("{}%", food_name);
    let second_best_search = best_search.replace(" ", "%");
    let food_name_search = format!("%{}", second_best_search);

    let foods = sqlx::query_as::<_, Food>("SELECT * FROM food WHERE name ILIKE $1 ORDER BY (CASE WHEN name ILIKE $2 THEN 1 WHEN name ILIKE $3 THEN 2 ELSE 3 END) ASC LIMIT 10")
        .bind(food_name_search)
        .bind(best_search)
        .bind(second_best_search)
        .fetch_all(dbpool)
        .await
        .ok()?;

    let mut search_foods : Vec<SearchFood> = vec![];
    for food in foods {
        let mut search_servings : Vec<SearchFoodServing> = vec![];

        let servings = sqlx::query_as::<_, Serving>("SELECT * FROM serving WHERE food_id = $1")
            .bind(food.id)
            .fetch_all(dbpool)
            .await
            .ok()?;

        for serving in servings {
            let mut nutrients : Vec<MealInfoNutrient> = vec![]; 

            if let None = serving.relative {
                nutrients = sqlx::query_as::<_, MealInfoNutrient>("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving_id = $1")
                    .bind(serving.id)
                    .fetch_all(dbpool)
                    .await
                    .ok()?; 
            }

            search_servings.push(SearchFoodServing {
                id: serving.id,
                amount: serving.amount,
                unit: serving.unit,
                nutrients,
                relative: serving.relative
            });
        }

        search_foods.push(SearchFood {
            id: food.id,
            name: food.name.clone(),
            servings: search_servings
        });
    }

    Some(search_foods)
}

pub async fn add_meal_serving(meal_id : i32, serving_id : i32, amount : f64, dbpool : &PgPool) -> Result<()> {
    sqlx::query("INSERT INTO meal_serving(meal_id, serving_id, amount) VALUES ($1, $2, $3)")
        .bind(meal_id)
        .bind(serving_id)
        .bind(amount)
        .execute(dbpool)
        .await?;

    Ok(())
}

pub async fn get_meal_serving_user_id(meal_serving_id : i32, dbpool : &PgPool) -> Option<i32> {
    let query_result = sqlx::query("SELECT meal_id FROM meal_serving WHERE id = $1")
        .bind(meal_serving_id)
        .fetch_one(dbpool)
        .await
        .ok()?;

    let meal_id : i32 = query_result.try_get("meal_id").ok()?;

    get_meal_user_id(meal_id, dbpool).await
}

pub async fn delete_meal_serving(meal_serving_id : i32, dbpool : &PgPool) -> Result<()> {
    sqlx::query("DELETE FROM meal_serving WHERE id = $1")
        .bind(meal_serving_id)
        .execute(dbpool)
        .await?;

    Ok(())
}
