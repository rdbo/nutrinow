/*
 * TODO:
     Add helper function for querying user ID from session ID
     Add helper function for getting session ID from cookie jar
     Add helper function for getting serving nutrients
     Limit amount of servings per food (too many would slow down the server)
 */

use rocket::{
    *,
    fs::{FileServer, relative, NamedFile},
    form::Form,
    serde::{Serialize, json::Json},
    http::{CookieJar}
};

use chrono::Datelike;

use rocket_db_pools::{
    Database, Connection,
    sqlx::{
        self, Row,
        types::chrono::{NaiveDate, Utc},
        types::uuid::Uuid
    }
};

use std::{
    path::PathBuf,
    collections::HashMap
};

mod helpers;

use helpers::{session_id_from_cookies, user_id_from_cookies, sha256str, diet_owner_id, meal_owner_id, user_information, calculate_age, duplicate_diet};

#[derive(Database)]
#[database("nutrinow")]
pub struct DbHandle(sqlx::PgPool);

// Register POST
#[derive(FromForm)]
struct RegisterData<'a> {
    name: &'a str,
    birthdate: &'a str,
    email: &'a str,
    password: &'a str,
    gender: &'a str,
    weight: f64
}

#[derive(Serialize)]
struct RegisterResponse {
    err: &'static str
}

impl RegisterResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

/*
 * TODO: 
 *   Check if e-mail is already registered
 *   Validate user input
 */
#[post("/register", data = "<data>")]
async fn api_register(data : Form<RegisterData<'_>>, mut db : Connection<DbHandle>) -> Json<RegisterResponse> {
    let birthdate = match NaiveDate::parse_from_str(data.birthdate, "%Y-%m-%d") {
        Ok(r) => r,
        _ => return Json(RegisterResponse::err("invalid birthdate format"))
    };
    let password_hash = sha256str(data.password);
    let create_account = async {
        sqlx::query(
            "INSERT INTO user_account(name, birthdate, email, password_hash, gender, weight) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(data.name)
            .bind(birthdate)
            .bind(data.email)
            .bind(password_hash)
            .bind(data.gender)
            .bind(data.weight)
            .execute(&mut *db)
            .await
    };
    if let Err(_) = create_account.await {
        return Json(RegisterResponse::err("failed to create account"));
    }
    Json(RegisterResponse::ok())
}

// Login POST
#[derive(FromForm)]
struct LoginData<'a> {
    email: &'a str,
    password: &'a str
}

#[derive(Serialize)]
struct LoginResponse {
    session_id : String,
    err: &'static str
}

impl LoginResponse {
    fn err(msg : &'static str) -> Self {
        Self { session_id: "".to_string(), err: msg }
    }

    fn ok(session_id : String) -> Self {
        Self { session_id : session_id, err: "" }
    }
}

#[post("/login", data = "<data>")]
async fn api_login(data : Form<LoginData<'_>>, mut db : Connection<DbHandle>) -> Json<LoginResponse> {
    let get_account_details = async {
        sqlx::query("SELECT password_hash, id FROM user_account WHERE email = $1")
            .bind(data.email)
            .fetch_one(&mut *db).await
    };
    let result = match get_account_details.await {
        Ok(r) => r,
        _ => return Json(LoginResponse::err("unable to find user account in database"))
    };

    let password_hash = match result.try_get::<&str, usize>(0) {
        Ok(val) => val,
        _ => return Json(LoginResponse::err("missing account password hash"))
    };

    let attempt_hash = sha256str(data.password);
    if attempt_hash != password_hash {
        return Json(LoginResponse::err("password does not match"));
    }

    let user_id = result.try_get::<i32, usize>(1).unwrap();
    let session_id = Uuid::new_v4();
    let expiry_date = Utc::now();
    let expiry_date = expiry_date.with_year(expiry_date.year() + 1);

    let gen_session = async {
        sqlx::query("INSERT INTO user_session(id, user_id, expiry_date) VALUES ($1, $2, $3)")
            .bind(session_id)
            .bind(user_id)
            .bind(expiry_date)
            .execute(&mut *db).await
    };

    if let Err(_) = gen_session.await {
        return Json(LoginResponse::err("failed to generate user session"));
    }

    Json(LoginResponse::ok(session_id.to_string()))
}

// Logout POST
#[post("/logout")]
async fn api_logout(cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) {
    let session_id = match session_id_from_cookies(cookies) {
        Ok(id) => id,
        Err(_) => return
    };

    sqlx::query("DELETE FROM user_session WHERE id = $1")
        .bind(session_id)
        .execute(&mut *db).await.ok();
}

// Food List Request

/*
 * TODO:
 *   Add sorting methods
 *   Limit results
 *   Remove unwraps
 */

#[derive(Serialize)]
struct ServingNutrient {
    name : String,
    amount : f64,
    unit: String
}

#[derive(Serialize)]
struct FoodInfo {
    id: i32,
    name : String,
    serving_id : i32,
    serving_base : f64,
    serving_amount : f64,
    serving_unit : String,
    base_nutrients : Vec<ServingNutrient>
}

#[derive(Serialize)]
struct FoodList {
    foods: Vec<FoodInfo>,
    err: &'static str
}

impl FoodList {
    fn err(msg : &'static str) -> Self {
        Self { foods: vec![], err: msg }
    }

    fn ok(food_list : Vec<FoodInfo>) -> Self {
        Self { foods: food_list, err: "" }
    }
}

#[get("/foods")]
async fn api_foods(mut db : Connection<DbHandle>) -> Json<FoodList> {
    let query_foods = async {
        sqlx::query("SELECT food.id AS food_id, food.name AS name, MIN(serving.id) AS serving_id FROM food JOIN serving ON serving.food_id = food.id GROUP BY food.id ORDER BY food.id")
            .fetch_all(&mut *db)
            .await
    };

    let foods = match query_foods.await {
        Ok(r) => r,
        Err(_) => return Json(FoodList::err("failed to query food entries"))
    };

    let mut food_list = FoodList::ok(vec![]);

    for food in foods {
        let food_id : i32 = food.try_get("food_id").unwrap();
        let name : String = food.try_get("name").unwrap();
        let serving_id : i32 = food.try_get("serving_id").unwrap();

        let query_serving = async {
            sqlx::query("SELECT id, amount, unit FROM serving WHERE serving.id = $1")
            .bind(serving_id)
            .fetch_one(&mut *db)
            .await
        };

        let serving = match query_serving.await {
            Ok(r) => r,
            Err(_) => continue
        };
        let serving_id : i32 = serving.try_get("id").unwrap();
        let serving_amount : f64 = serving.try_get("amount").unwrap();
        let serving_base = serving_amount;
        let serving_unit : String = serving.try_get("unit").unwrap();

        let query_nutrients = async {
            sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving_id = $1")
            .bind(serving_id)
            .fetch_all(&mut *db)
            .await
        };

        let nutrients = match query_nutrients.await {
            Ok(r) => r,
            Err(_) => continue
        };

        let mut nutrient_list : Vec<ServingNutrient> = vec![];
        for nutrient in nutrients {
            let nutrient_name : String = nutrient.try_get("name").unwrap();
            let nutrient_amount : f64 = nutrient.try_get("amount").unwrap();
            let nutrient_unit : String = nutrient.try_get("unit").unwrap();
            let nutrient_item = ServingNutrient {
                name : nutrient_name,
                amount : nutrient_amount,
                unit : nutrient_unit
            };
            nutrient_list.push(nutrient_item);
        }

        let food_item = FoodInfo {
            id : food_id,
            name,
            serving_id,
            serving_base,
            serving_amount,
            serving_unit,
            base_nutrients : nutrient_list
        };

        food_list.foods.push(food_item);
    }

    Json(food_list)
}

// Food Request
/*
#[get("/nutrients/<serving_id>")]
async fn api_food(food_id : i32, mut db : Connection<DbHandle>) -> Json {
    let query_food = async {
        sqlx::query("SELECT food.name, serving.amount, serving.unit, nutrient.name, serving_nutrient.amount, nutrient.unit FROM food JOIN serving ON serving.food_id = food.id JOIN serving_nutrient ON serving_nutrient.serving_id = serving.id JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE food.id = $1")
        .bind(id)
        .execute(&mut *db)
        .await
    };

    let food = match query_food.await {
        Ok(r) => r,
        Err(_) => return Json("failed to get food information")
    };

    Json("")
}
*/

// Diets Request
#[derive(Serialize)]
struct DietNutrient {
    name : String,
    min_amount : Option<f64>,
    max_amount : Option<f64>,
    unit : String,
    relative : bool
}

#[derive(Serialize)]
struct DietInfo {
    id : i32,
    name : String,
    desired_nutrition : Vec<DietNutrient>
}

#[derive(Serialize)]
struct DietsResponse {
    diets : Vec<DietInfo>,
    err: &'static str
}

impl DietsResponse {
    fn err(msg : &'static str) -> Self {
        Self { diets: vec![], err: msg }
    }

    fn ok(diet_list : Vec<DietInfo>) -> Self {
        Self { diets: diet_list, err: "" }
    }
}

#[get("/diets")]
async fn api_diets(cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<DietsResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(DietsResponse::err(s))
    };

    let query_diets = async {
        sqlx::query("SELECT id, name FROM diet WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&mut *db)
            .await
    };

    let diets = match query_diets.await {
        Ok(r) => r,
        Err(_) => return Json(DietsResponse::err("failed to query diets"))
    };

    let mut diet_list : Vec<DietInfo> = vec![];
    for diet in diets {
        let diet_id : i32 = diet.try_get("id").unwrap();
        let diet_name : String = diet.try_get("name").unwrap();

        diet_list.push(DietInfo { id: diet_id, name: diet_name, desired_nutrition: vec![] });
    }

    Json(DietsResponse::ok(diet_list))
}

// Diet Nutrition Request
#[derive(Serialize)]
struct DietNutritionResponse {
    nutrition : Vec<DietNutrient>,
    err : &'static str
}

impl DietNutritionResponse {
    fn err(msg : &'static str) -> Self {
        Self { nutrition: vec![], err: msg }
    }

    fn ok(nutrition : Vec<DietNutrient>) -> Self {
        Self { nutrition, err: "" }
    }
}

#[get("/diet_nutrition/<diet_id>")]
async fn api_diet_nutrition(diet_id : i32, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<DietNutritionResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(DietNutritionResponse::err(s))
    };

    let diet_owner_id = match diet_owner_id(diet_id, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(DietNutritionResponse::err(s))
    };

    if user_id != diet_owner_id {
        return Json(DietNutritionResponse::err("User does not own diet"));
    }

    let query_diet_nutrition = async {
        sqlx::query("SELECT name, min_intake, max_intake, unit, relative FROM diet_nutrition JOIN nutrient ON nutrient.id = diet_nutrition.nutrient_id WHERE diet_id = $1")
            .bind(diet_id)
            .fetch_all(&mut *db)
            .await
    };

    let diet_nutrition = match query_diet_nutrition.await {
        Ok(r) => r,
        Err(_) => return Json(DietNutritionResponse::err("Failed to query diet nutrition"))
    };

    let mut diet_nutrients : Vec<DietNutrient> = vec![];
    for nutrient in diet_nutrition {
        let nutrient_name : String = nutrient.try_get("name").unwrap();
        let nutrient_min_intake : Option<f64> = nutrient.try_get("min_intake").ok();
        let nutrient_max_intake : Option<f64> = nutrient.try_get("max_intake").ok();
        let nutrient_unit : String = nutrient.try_get("unit").unwrap();
        let nutrient_relative : bool = nutrient.try_get("relative").unwrap();

        let diet_nutrient = DietNutrient {
            name: nutrient_name,
            min_amount: nutrient_min_intake,
            max_amount: nutrient_max_intake,
            unit: nutrient_unit,
            relative: nutrient_relative
        };
        diet_nutrients.push(diet_nutrient);
    }

    Json(DietNutritionResponse::ok(diet_nutrients))
}

// Delete Diet Request
#[derive(FromForm)]
struct DeleteDietForm {
    diet_id : i32
}

#[derive(Serialize)]
struct DeleteDietResponse {
    err : &'static str
}

impl DeleteDietResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

#[post("/delete_diet", data = "<data>")]
async fn api_delete_diet(data : Form<DeleteDietForm>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<DeleteDietResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(DeleteDietResponse::err(s))
    };

    let diet_owner_id = match diet_owner_id(data.diet_id, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(DeleteDietResponse::err(s))
    };

    if user_id != diet_owner_id {
        return Json(DeleteDietResponse::err("User does not own diet"));
    }

    let delete_diet = async {
        sqlx::query("DELETE FROM meal_serving WHERE meal_id IN (SELECT meal_serving.id FROM meal_serving JOIN meal ON meal.id = meal_serving.meal_id WHERE diet_id = $1)")
            .bind(data.diet_id)
            .execute(&mut *db)
            .await
            .ok();

        sqlx::query("DELETE FROM meal WHERE diet_id = $1")
            .bind(data.diet_id)
            .execute(&mut *db)
            .await
            .ok();

        sqlx::query("DELETE FROM diet_nutrition WHERE diet_id = $1")
            .bind(data.diet_id)
            .execute(&mut *db)
            .await
            .ok();

        match sqlx::query("DELETE FROM diet WHERE id = $1")
            .bind(data.diet_id)
            .execute(&mut *db)
            .await {
            Ok(_) => { },
            Err(_) => return Err(())
        };

        Ok(())
    };

    match delete_diet.await {
        Ok(_) => Json(DeleteDietResponse::ok()),
        Err(_) => Json(DeleteDietResponse::err("Failed to delete diet"))
    }
}

// New Diet Request
#[derive(FromForm)]
struct NewDietForm<'a> {
    diet_name: &'a str
}

#[derive(Serialize)]
struct NewDietResponse {
    err: &'static str
}

impl NewDietResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

#[post("/new_diet", data = "<data>")]
async fn api_new_diet(data : Form<NewDietForm<'_>>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<NewDietResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(NewDietResponse::err(s))
    };

    let user_info = match user_information(user_id, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(NewDietResponse::err(s))
    };

    let user_age = calculate_age(&user_info.birthdate);

    let query_new_diet = async {
        sqlx::query("INSERT INTO diet(name, user_id) VALUES ($1, $2) RETURNING id")
            .bind(data.diet_name)
            .bind(user_id)
            .fetch_one(&mut *db)
            .await
    };

    let mut default_nutrition = HashMap::new();
    /* Daily Dietary Intake Estimates (for teens and adults only!). Main Source (consumer fact sheets): https://ods.od.nih.gov */
    /* TODO: Add these values to database, they don't belong in source code! */
    default_nutrition.insert("Protein", (1.0, None, true));
    default_nutrition.insert("Carbohydrates", (2.5, None, true));
    default_nutrition.insert("Fats", (1.0, None, true));
    default_nutrition.insert("Sugars", (0.0, Some(30.0), false)); // TODO: Adjust
    default_nutrition.insert("Fiber", (
        if user_info.gender == "M" {
            38.0
        } else {
            25.0
        }
    , None, false));
    default_nutrition.insert("Saturated Fat", (10.0, None, false));
    default_nutrition.insert("Unsaturated Fat", (20.0, None, false));
    default_nutrition.insert("Trans Fat", (2.0, None, false));
    default_nutrition.insert("Vitamin A", (
        if user_age < 18 {
            600.0
        } else {
            if user_info.gender == "M" { 900.0 }
            else { 700.0 }
        }
    , None, false));
    default_nutrition.insert("Vitamin B1", (if user_info.gender == "M" { 1.2 } else { 1.1 }, None, false));
    default_nutrition.insert("Vitamin B2", (if user_info.gender == "M" { 1.3 } else { 1.1 }, None, false));
    default_nutrition.insert("Vitamin B3", (if user_info.gender == "M" { 16.0 } else { 14.0 }, None, false));
    default_nutrition.insert("Vitamin B5", (5.0, None, false));
    default_nutrition.insert("Vitamin B6", (1.3, None, false));
    default_nutrition.insert("Vitamin B7", (if user_age <= 18 { 25.0 } else { 3.0 }, None, false));
    default_nutrition.insert("Vitamin B9", (400.0, None, false));
    default_nutrition.insert("Vitamin B12", (2.4, None, false));
    default_nutrition.insert("Vitamin C", (
        if user_info.gender == "M" {
            if user_age <= 18 {
                75.0
            } else {
                90.0
            }
        } else {
            if user_age <= 18 {
                65.0
            } else {
                75.0
            }
        }
    , None, false));
    default_nutrition.insert("Vitamin D", (15.0, None, false));
    default_nutrition.insert("Vitamin E", (15.0, None, false));
    default_nutrition.insert("Vitamin K", (
        if user_age <= 18 {
            75.0
        } else {
            if user_info.gender == "M" {
                120.0
            } else {
                90.0
            }
        }
    , None, false));
    default_nutrition.insert("Calcium", (
        if user_age <= 18 {
            1300.0
        } else {
            1000.0
        }
    , None, false));
    default_nutrition.insert("Iron", (
        if user_age <= 18 {
            if user_info.gender == "M" {
                11.0
            } else {
                15.0
            }
        } else {
            if user_info.gender == "M" {
                8.0
            } else {
                18.0
            }
        }
    , None, false));
    default_nutrition.insert("Magnesium", (
        if user_info.gender == "M" {
            410.0
        } else {
            if user_age <= 18 {
                360.0
            } else {
                310.0
            }
        }
    , None, false));
    default_nutrition.insert("Phosphorus", (
        if user_age <= 18 {
            1250.0
        } else {
            700.0
        }
    , None, false));
    default_nutrition.insert("Potassium", (
        if user_age <= 18 {
            if user_info.gender == "M" {
                3000.0
            } else {
                2300.0
            }
        } else {
            if user_info.gender == "M" {
                3400.0
            } else {
                2600.0
            }
        }
    , None, false));
    default_nutrition.insert("Sodium", (1500.0, None, false));
    default_nutrition.insert("Zinc", (
        if user_info.gender == "M" {
            11.0
        } else {
            8.0
        }
    , None, false));
    default_nutrition.insert("Copper", (900.0, None, false));
    default_nutrition.insert("Manganese", (
        if user_info.gender == "M" {
            400.0
        } else {
            310.0
        }
    , None, false));
    default_nutrition.insert("Selenium", (55.0, None, false));
    default_nutrition.insert("Water", (0.033 * 1000.0 /* convert from l to ml */, None, true));

    let new_diet = match query_new_diet.await {
        Ok(r) => r,
        Err(_) => return Json(NewDietResponse::err("Failed to add new diet"))
    };

    let diet_id : i32 = new_diet.try_get("id").unwrap();

    for (key, value) in default_nutrition {
        sqlx::query("INSERT INTO diet_nutrition(diet_id, nutrient_id, min_intake, max_intake, relative) VALUES ($1, (SELECT id FROM nutrient WHERE name = $2 LIMIT 1), $3, $4, $5)")
            .bind(diet_id)
            .bind(key)
            .bind(value.0)
            .bind(value.1)
            .bind(value.2)
            .execute(&mut *db)
            .await
            .ok();
    }

    Json(NewDietResponse::ok())
}

// Edit Diet Request
#[derive(FromForm)]
struct EditDietForm<'a> {
    diet_id : i32,
    diet_name : &'a str
}

#[derive(Serialize)]
struct EditDietResponse {
    err: &'static str
}

impl EditDietResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

#[post("/edit_diet", data = "<data>")]
async fn api_edit_diet(data : Form<EditDietForm<'_>>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<EditDietResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(EditDietResponse::err(s))
    };

    let query_edit_diet = async {
        sqlx::query("UPDATE diet SET name = $1 WHERE id = $2 AND user_id = $3")
            .bind(data.diet_name)
            .bind(data.diet_id)
            .bind(user_id)
            .execute(&mut *db)
            .await
    };

    match query_edit_diet.await {
        Ok(_) => Json(EditDietResponse::ok()),
        Err(_) => Json(EditDietResponse::err("failed to edit diet"))
    }
}

// Meals Request
#[derive(Serialize)]
struct MealFoodInfo {
    id: i32,
    name : String,
    meal_serving_id : i32,
    serving_id : i32,
    serving_base : f64,
    serving_amount : f64,
    serving_unit : String,
    base_nutrients : Vec<ServingNutrient>
}

#[derive(Serialize)]
struct MealInfo {
    id : i32,
    name : String,
    foods : Vec<MealFoodInfo>
}

#[derive(Serialize)]
struct MealsResponse {
    meals : Vec<MealInfo>,
    err : &'static str
}

impl MealsResponse {
    fn err(msg : &'static str) -> Self {
        Self { meals: vec![], err: msg }
    }

    fn ok(meals_info : Vec<MealInfo>) -> Self {
        Self { meals: meals_info, err: "" }
    }
}

#[get("/meals/<diet_id>")]
async fn api_meals(diet_id : i32, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<MealsResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(MealsResponse::err(s))
    };

    let query_meals = async {
        sqlx::query("SELECT meal.id AS id, meal.name AS name FROM meal JOIN diet ON diet.id = meal.diet_id WHERE diet.id = $1 AND diet.user_id = $2")
            .bind(diet_id)
            .bind(user_id)
            .fetch_all(&mut *db)
            .await
    };

    let meals = match query_meals.await {
        Ok(r) => r,
        Err(_) => return Json(MealsResponse::err("Failed to query meals from diet"))
    };

    let mut meals_info : Vec<MealInfo> = vec![];
    for meal in meals {
        let meal_id : i32 = meal.try_get("id").unwrap();
        let meal_name : String = meal.try_get("name").unwrap();

        let query_foods = async {
            sqlx::query("SELECT food.id AS id, food.name AS name, meal_serving.id AS meal_serving_id, serving.id AS serving_id, serving.amount AS serving_base, meal_serving.amount AS amount, serving.unit AS unit, serving.relative AS relative FROM meal_serving JOIN serving ON meal_serving.serving_id = serving.id JOIN food ON serving.food_id = food.id WHERE meal_serving.meal_id = $1")
                .bind(meal_id)
                .fetch_all(&mut *db)
                .await
        };

        let foods = match query_foods.await {
            Ok(r) => r,
            Err(_) => continue
        };

        let mut foods_info : Vec<MealFoodInfo> = vec![];
        for food in foods {
            let food_id : i32 = food.try_get("id").unwrap();
            let food_name : String = food.try_get("name").unwrap();
            let meal_serving_id : i32 = food.try_get("meal_serving_id").unwrap();
            let serving_id : i32 = food.try_get("serving_id").unwrap();
            let mut serving_base : f64 = food.try_get("serving_base").unwrap();
            let serving_amount : f64 = food.try_get("amount").unwrap();
            let serving_unit : String = food.try_get("unit").unwrap();
            let serving_relative : Result<i32, _> = food.try_get("relative");
            let mut serving_rel_amount : f64 = 0.0;

            if let Ok(_) = serving_relative {
                serving_base = 1.0;

                let row = match sqlx::query("SELECT amount FROM serving WHERE serving.id = $1")
                    .bind(serving_id)
                    .fetch_one(&mut *db)
                    .await {
                    Ok(r) => r,
                    Err(_) => continue
                };

                serving_rel_amount = row.try_get("amount").unwrap();
            }

            let query_nutrients = async {
                if let Ok(id) = serving_relative {
                    sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit, serving.amount AS serving_base_amount FROM serving_nutrient JOIN serving ON serving.id = serving_nutrient.serving_id JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving.id = $1")
                        .bind(id)
                        .fetch_all(&mut *db)
                        .await
                } else {
                    sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN serving ON serving.id = serving_nutrient.serving_id JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving.id = $1")
                        .bind(serving_id)
                        .fetch_all(&mut *db)
                        .await
                }
            };

            let nutrients = match query_nutrients.await {
                Ok(r) => r,
                Err(_) => continue
            };

            let mut base_nutrients : Vec<ServingNutrient> = vec![];
            for nutrient in nutrients {
                let nutrient_name : String = nutrient.try_get("name").unwrap();
                let mut nutrient_amount : f64 = nutrient.try_get("amount").unwrap();
                let nutrient_unit : String = nutrient.try_get("unit").unwrap();

                if let Ok(_) = serving_relative {
                    let serving_base_amount : f64 = nutrient.try_get("serving_base_amount").unwrap();
                    nutrient_amount *= serving_rel_amount / serving_base_amount;
                }

                let nutrient_info = ServingNutrient {
                    name: nutrient_name,
                    amount: nutrient_amount,
                    unit: nutrient_unit
                };
                base_nutrients.push(nutrient_info);
            }

            let food = MealFoodInfo {
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

        let meal = MealInfo {
            id: meal_id,
            name: meal_name,
            foods: foods_info
        };
        meals_info.push(meal);
    }
 
    Json(MealsResponse::ok(meals_info))
}

// User Information Request
#[derive(Serialize)]
struct UserResponse {
    name : String,
    birthdate : String,
    gender : String,
    weight : f64,
    err : &'static str
}

impl UserResponse {
    fn err(msg : &'static str) -> Self {
        Self { name: "".to_string(), birthdate: "".to_string(), gender: "".to_string(), weight: 0.0, err: msg }
    }

    fn ok(name : String, birthdate : String, gender : String, weight : f64) -> Self {
        Self { name, birthdate, gender, weight, err: "" }
    }
}

#[get("/user")]
async fn api_user(cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<UserResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(UserResponse::err(s))
    };

    let query_user_info = async {
        sqlx::query("SELECT name, birthdate, gender, weight FROM user_account WHERE id = $1")
            .bind(user_id)
            .fetch_one(&mut *db)
            .await
    };

    let user_info = match query_user_info.await {
        Ok(r) => r,
        Err(_) => return Json(UserResponse::err("Failed to query user information"))
    };

    let user_name : String = user_info.try_get("name").unwrap();
    let user_birthdate : NaiveDate = user_info.try_get("birthdate").unwrap();
    let user_birthdate = user_birthdate.to_string();
    let user_gender : String = user_info.try_get("gender").unwrap();
    let user_weight : f64 = user_info.try_get("weight").unwrap();

    Json(UserResponse::ok(user_name, user_birthdate, user_gender, user_weight))
}

// Add Meal Request
#[derive(FromForm)]
struct AddMealForm<'a> {
    diet_id : i32,
    meal_name : &'a str
}

#[derive(Serialize)]
struct AddMealResponse {
    meal : MealInfo,
    err : &'static str
}

impl AddMealResponse {
    fn err(msg : &'static str) -> Self {
        Self { meal: MealInfo { id: 0, name: "".to_string(), foods: vec![] }, err: msg }
    }

    fn ok(meal_info : MealInfo) -> Self {
        Self { meal: meal_info, err: "" }
    }
}

#[post("/add_meal", data = "<data>")]
async fn api_add_meal(data : Form<AddMealForm<'_>>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<AddMealResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(AddMealResponse::err(s))
    };

    // TODO: Make querying diet owner a function
    let diet_owner_id = match diet_owner_id(data.diet_id, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(AddMealResponse::err(s))
    };

    if user_id != diet_owner_id {
        return Json(AddMealResponse::err("User does not own diet"));
    }

    let query_add_meal = async {
        sqlx::query("INSERT INTO meal (diet_id, name) VALUES ($1, $2) RETURNING id")
            .bind(data.diet_id)
            .bind(data.meal_name)
            .fetch_one(&mut *db)
            .await
    };

    let meal_row = match query_add_meal.await {
        Ok(r) => r,
        Err(_) => return Json(AddMealResponse::err("Failed to add meal to diet"))
    };

    let meal_id : i32 = meal_row.try_get("id").unwrap();

    Json(AddMealResponse::ok(MealInfo { id: meal_id, name: data.meal_name.to_string(), foods: vec![] }))
}

// Delete Meal Request
#[derive(FromForm)]
struct DeleteMealForm {
    meal_id : i32
}

#[derive(Serialize)]
struct DeleteMealResponse {
    err: &'static str
}

impl DeleteMealResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

#[post("/delete_meal", data = "<data>")]
async fn api_delete_meal(data : Form<DeleteMealForm>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<DeleteMealResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(DeleteMealResponse::err(s))
    };

    let diet_owner_id = match meal_owner_id(data.meal_id, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(DeleteMealResponse::err(s))
    };

    if user_id != diet_owner_id {
        return Json(DeleteMealResponse::err("User does not own diet"));
    }

    // TODO: Make function to delete meal
    let query_delete_foods = async {
        sqlx::query("DELETE FROM meal_serving WHERE meal_id = $1")
            .bind(data.meal_id)
            .execute(&mut *db)
            .await
    };

    match query_delete_foods.await {
        Ok(_) => {  },
        Err(_) => return Json(DeleteMealResponse::err("Failed to delete meal servings"))
    };

    let query_delete_meal = async {
        sqlx::query("DELETE FROM meal WHERE id = $1")
            .bind(data.meal_id)
            .execute(&mut *db)
            .await
    };

    match query_delete_meal.await {
        Ok(_) => Json(DeleteMealResponse::ok()),
        Err(_) => Json(DeleteMealResponse::err("Failed to delete meal"))
    }
}

// Nutrients Request
#[derive(Serialize)]
struct Nutrient {
    id : i32,
    name : String,
    unit : String
}

#[derive(Serialize)]
struct NutrientsResponse {
    nutrients : Vec<Nutrient>,
    err : &'static str
}

impl NutrientsResponse {
    fn err(msg : &'static str) -> Self {
        Self { nutrients: vec![], err: msg }
    }

    fn ok(nutrient_list : Vec<Nutrient>) -> Self {
        Self { nutrients: nutrient_list, err: "" }
    }
}

#[get("/nutrients")]
async fn api_nutrients(mut db : Connection<DbHandle>) -> Json<NutrientsResponse> {
    let query_nutrients = async {
        sqlx::query("SELECT id, name, unit FROM nutrient")
            .fetch_all(&mut *db)
            .await
    };

    let nutrients = match query_nutrients.await {
        Ok(r) => r,
        Err(_) => return Json(NutrientsResponse::err("Failed to query nutrients"))
    };

    let mut nutrient_list : Vec<Nutrient> = vec![];
    for nutrient in nutrients {
        let nutrient_id : i32 = nutrient.try_get("id").unwrap();
        let nutrient_name : String = nutrient.try_get("name").unwrap();
        let nutrient_unit : String = nutrient.try_get("unit").unwrap();

        let nutrient_item = Nutrient { id: nutrient_id, name: nutrient_name, unit: nutrient_unit };
        nutrient_list.push(nutrient_item);
    }

    Json(NutrientsResponse::ok(nutrient_list))
}

// Food Search Request
#[derive(Serialize)]
struct Serving {
    id : i32,
    amount : f64,
    unit : String,
    nutrients : Vec<ServingNutrient>,
    relative : Option<i32> // may be NULL, or a serving ID
}

#[derive(Serialize)]
struct Food {
    id: i32,
    name : String,
    servings : Vec<Serving>
}

#[derive(Serialize)]
struct FoodSearchResponse {
    matches : Vec<Food>,
    err : &'static str
}

impl FoodSearchResponse {
    fn err(msg : &'static str) -> Self {
        Self { matches: vec![], err: msg }
    }

    fn ok(matches : Vec<Food>) -> Self {
        Self { matches, err: "" }
    }
}

#[get("/food_search/<food_name>")]
async fn api_food_search(food_name : String, mut db : Connection<DbHandle>) -> Json<FoodSearchResponse> {
    // treat search string for ILIKE statement from PostgreSQL
    // it will replace all spaces with % to match anything in between them
    // and will also match things at the center and end of names.
    // The input 'Chicken Breast' will become '%Chicken%Breast%', and ILIKE
    // will ignore the case when searching. 
    let mut best_search = food_name.clone();
    best_search.push('%');

    let second_best_search = best_search.replace(" ", "%");

    let mut food_name_search = second_best_search.clone();
    food_name_search.insert(0, '%');

    let query_food_matches = async {
        sqlx::query("SELECT id, name FROM food WHERE name ILIKE $1 ORDER BY (CASE WHEN name ILIKE $2 THEN 1 WHEN name ILIKE $3 THEN 2 ELSE 3 END) ASC LIMIT 50")
            .bind(food_name_search)
            .bind(best_search)
            .bind(second_best_search)
            .fetch_all(&mut *db)
            .await
    };

    let food_matches = match query_food_matches.await {
        Ok(r) => r,
        Err(_) => return Json(FoodSearchResponse::err("Failed to query food matches"))
    };

    let mut food_list : Vec<Food> = vec![];
    for food_match in food_matches {
        let food_id : i32 = food_match.try_get("id").unwrap();
        let food_name : String = food_match.try_get("name").unwrap();

        let query_food_servings = async {
            sqlx::query("SELECT id, amount, unit, relative FROM serving WHERE food_id = $1")
                .bind(food_id)
                .fetch_all(&mut *db)
                .await
        };
        let food_servings = match query_food_servings.await {
            Ok(r) => r,
            Err(_) => continue
        };

        let mut food_serving_list : Vec<Serving> = vec![];
        for food_serving in food_servings {
            let serving_id : i32 = food_serving.try_get("id").unwrap();
            let serving_amount : f64 = food_serving.try_get("amount").unwrap();
            let serving_unit : String = food_serving.try_get("unit").unwrap();
            let serving_relative : Option<i32> = food_serving.try_get("relative").unwrap();

            let query_nutrients = async {
                sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving_id = $1")
                .bind(serving_id)
                .fetch_all(&mut *db)
                .await
            };

            let nutrients = match query_nutrients.await {
                Ok(r) => r,
                Err(_) => continue
            };

            let mut nutrient_list : Vec<ServingNutrient> = vec![];
            if let None = serving_relative {
                for nutrient in nutrients {
                    let nutrient_name : String = nutrient.try_get("name").unwrap();
                    let nutrient_amount : f64 = nutrient.try_get("amount").unwrap();
                    let nutrient_unit : String = nutrient.try_get("unit").unwrap();
                    let nutrient_item = ServingNutrient {
                        name : nutrient_name,
                        amount : nutrient_amount,
                        unit : nutrient_unit
                    };
                    nutrient_list.push(nutrient_item);
                }
            }

            let serving = Serving {
                id: serving_id,
                amount: serving_amount,
                unit: serving_unit,
                nutrients: nutrient_list,
                relative: serving_relative
            };

            food_serving_list.push(serving);
        }

        let food = Food {
            id: food_id,
            name: food_name,
            servings: food_serving_list
        };
        food_list.push(food);
    }

    Json(FoodSearchResponse::ok(food_list))
}

// Add to Meal Request
#[derive(FromForm)]
struct AddMealServingForm {
    meal_id : i32,
    serving_id : i32,
    amount : f64
}

#[derive(Serialize)]
struct AddMealServingResponse {
    err : &'static str
}

impl AddMealServingResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

#[post("/add_meal_serving", data = "<data>")]
async fn api_add_meal_serving(data : Form<AddMealServingForm>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<AddMealServingResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(AddMealServingResponse::err(s))
    };

    let meal_owner_id = match meal_owner_id(data.meal_id, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(AddMealServingResponse::err(s))
    };

    if meal_owner_id != user_id {
        return Json(AddMealServingResponse::err("User does not own meal"));
    }

    // if the serving doesn't exist, this function will fail
    let query_add_meal_serving = async {
        sqlx::query("INSERT INTO meal_serving(meal_id, serving_id, amount) VALUES ($1, $2, $3)")
            .bind(data.meal_id)
            .bind(data.serving_id)
            .bind(data.amount)
            .execute(&mut *db)
            .await
    };

    match query_add_meal_serving.await {
        Ok(_) => Json(AddMealServingResponse::ok()),
        Err(_) => Json(AddMealServingResponse::err("Failed to add serving to meal"))
    }
}

// Duplicate Diet Request
#[derive(FromForm)]
struct DuplicateDietForm<'a> {
    diet_id : i32,
    diet_name : &'a str
}

#[derive(Serialize)]
struct DuplicateDietResponse {
    err : &'static str
}

impl DuplicateDietResponse {
    fn err(msg : &'static str) -> Self {
        Self { err: msg }
    }

    fn ok() -> Self {
        Self { err: "" }
    }
}

#[post("/duplicate_diet", data = "<data>")]
async fn api_duplicate_diet(data : Form<DuplicateDietForm<'_>>, cookies : &CookieJar<'_>, mut db : Connection<DbHandle>) -> Json<DuplicateDietResponse> {
    let user_id = match user_id_from_cookies(cookies, &mut *db).await {
        Ok(r) => r,
        Err(s) => return Json(DuplicateDietResponse::err(s))
    };

    let diet_owner_id = match diet_owner_id(data.diet_id, &mut *db).await {
        Ok(id) => id,
        Err(s) => return Json(DuplicateDietResponse::err(s))
    };

    if user_id != diet_owner_id {
        return Json(DuplicateDietResponse::err("User does not own diet"));
    }

    match duplicate_diet(user_id, data.diet_id, data.diet_name, &mut *db).await {
        Ok(_) => Json(DuplicateDietResponse::ok()),
        Err(s) => Json(DuplicateDietResponse::err(s))
    }
}

// Handle Vue routes that are not static files
#[get("/<_..>", rank = 12)]
async fn vue_routes() -> Option<NamedFile> {
    let index_path = PathBuf::from(relative!("static")).join("index.html");
    NamedFile::open(index_path).await.ok()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(DbHandle::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![vue_routes])
        .mount("/api", routes![api_login, api_register, api_logout, api_foods, api_diets, api_delete_diet, api_new_diet, api_edit_diet, api_meals, api_user, api_add_meal, api_delete_meal, api_nutrients, api_diet_nutrition, api_food_search, api_add_meal_serving, api_duplicate_diet])
}
