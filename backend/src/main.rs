/*
 * TODO: Add helper function for querying user ID from session ID
 */

use rocket::{
    *,
    fs::{FileServer, relative, NamedFile},
    form::Form,
    serde::{Serialize, json::Json}
};

use chrono::Datelike;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{
    self, Row,
    types::chrono::{NaiveDate, Utc},
    types::uuid::Uuid
};

use sha2::{Sha256, Digest};

use std::path::PathBuf;

use std::str::FromStr;

fn sha256str(string : &str) -> String {
    let mut hash = Sha256::new();
    hash.update(string);
    format!("{:x}", hash.finalize())
}

#[derive(Database)]
#[database("nutrinow")]
struct DbHandler(sqlx::PgPool);

// Register POST
#[derive(FromForm)]
struct RegisterData<'a> {
    name: &'a str,
    birthdate: &'a str,
    email: &'a str,
    password: &'a str,
    gender: &'a str
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
async fn api_register(data : Form<RegisterData<'_>>, mut db : Connection<DbHandler>) -> Json<RegisterResponse> {
    let birthdate = match NaiveDate::parse_from_str(data.birthdate, "%Y-%m-%d") {
        Ok(r) => r,
        _ => return Json(RegisterResponse::err("invalid birthdate format"))
    };
    let password_hash = sha256str(data.password);
    let create_account = async {
        sqlx::query(
            "INSERT INTO user_account(name, birthdate, email, password_hash, gender) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(data.name)
            .bind(birthdate)
            .bind(data.email)
            .bind(password_hash)
            .bind(data.gender)
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
async fn api_login(data : Form<LoginData<'_>>, mut db : Connection<DbHandler>) -> Json<LoginResponse> {
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
#[derive(FromForm)]
struct LogoutData<'a> {
    session_id : &'a str
}

#[post("/logout", data = "<data>")]
async fn api_logout(data : Form<LogoutData<'_>>, mut db : Connection<DbHandler>) {
    let session_id = match Uuid::from_str(data.session_id) {
        Ok(r) => r,
        _ => return
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
    serving_amount : f64,
    serving_unit : String,
    calories : ServingNutrient,
    nutrients : Vec<ServingNutrient>
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
async fn api_foods(mut db : Connection<DbHandler>) -> Json<FoodList> {
    let query_foods =  async {
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
        let serving_unit : String = serving.try_get("unit").unwrap();

        let query_calories = async {
            sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving_id = $1 AND name = 'Calories'")
            .bind(serving_id)
            .fetch_one(&mut *db)
            .await
        };

        let calories = match query_calories.await {
            Ok(r) => r,
            Err(_) => continue
        };

        let calories_name : String = calories.try_get("name").unwrap();
        let calories_amount : f64 = calories.try_get("amount").unwrap();
        let calories_unit : String = calories.try_get("unit").unwrap();

        let query_nutrients = async {
            sqlx::query("SELECT nutrient.name AS name, serving_nutrient.amount AS amount, nutrient.unit AS unit FROM serving_nutrient JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE serving_id = $1 AND name IN ('Carbohydrates', 'Protein', 'Fats')")
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

        let calories = ServingNutrient {
            name : calories_name,
            amount : calories_amount,
            unit : calories_unit
        };

        let food_item = FoodInfo {
            id : food_id,
            name,
            serving_id,
            serving_amount,
            serving_unit,
            calories,
            nutrients : nutrient_list
        };

        food_list.foods.push(food_item);
    }

    Json(food_list)
}

// Food Request
/*
#[get("/nutrients/<serving_id>")]
async fn api_food(food_id : i32, mut db : Connection<DbHandler>) -> Json {
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
struct DietInfo {
    id : i32,
    name : String
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

#[get("/diets/<session_id>")]
async fn api_diets(session_id : String, mut db : Connection<DbHandler>) -> Json<DietsResponse> {
    let session_uuid = match Uuid::from_str(&session_id) {
        Ok(r) => r,
        Err(_) => return Json(DietsResponse::err("invalid session id"))
    };

    let query_user_id = async {
        sqlx::query("SELECT user_id FROM user_session WHERE id = $1")
            .bind(session_uuid)
            .fetch_one(&mut *db)
            .await
    };
    let user_id = match query_user_id.await {
        Ok(r) => r,
        Err(_) => return Json(DietsResponse::err("failed to query user id"))
    };
    let user_id : i32 = user_id.try_get("user_id").unwrap();

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
        diet_list.push(DietInfo { id: diet_id, name: diet_name });
    }

    Json(DietsResponse::ok(diet_list))
}

// Delete Diet Request
#[derive(FromForm)]
struct DeleteDietForm<'a> {
    session_id : &'a str,
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
async fn api_delete_diet(data : Form<DeleteDietForm<'_>>, mut db : Connection<DbHandler>) -> Json<DeleteDietResponse> {
    let session_uuid = match Uuid::from_str(data.session_id) {
        Ok(r) => r,
        Err(_) => return Json(DeleteDietResponse::err("invalid session id"))
    };

    let query_user_id = async {
        sqlx::query("SELECT user_id FROM user_session WHERE id = $1")
            .bind(session_uuid)
            .fetch_one(&mut *db)
            .await
    };

    let user_id = match query_user_id.await {
        Ok(r) => r,
        Err(_) => return Json(DeleteDietResponse::err("failed to query user id"))
    };
    let user_id : i32 = user_id.try_get("user_id").unwrap();

    let query_diet_owner_id = async {
        sqlx::query("SELECT user_id FROM diet WHERE id = $1")
            .bind(data.diet_id)
            .fetch_one(&mut *db)
            .await
    };

    let diet_owner_id = match query_diet_owner_id.await {
        Ok(r) => r,
        Err(_) => return Json(DeleteDietResponse::err("failed to query diet owner id"))
    };
    let diet_owner_id : i32 = diet_owner_id.try_get("user_id").unwrap();

    if user_id != diet_owner_id {
        return Json(DeleteDietResponse::err("user does not own diet"));
    }

    let delete_diet = async {
        match sqlx::query("DELETE FROM diet_meal WHERE diet_id = $1")
            .bind(data.diet_id)
            .execute(&mut *db)
            .await {
            Ok(_) => { },
            Err(_) => return Err(())
        };

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
        Err(_) => Json(DeleteDietResponse::err("failed to delete diet"))
    }
}

// New Diet Request
#[derive(FromForm)]
struct NewDietForm<'a> {
    session_id: &'a str,
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
async fn api_new_diet(data : Form<NewDietForm<'_>>, mut db : Connection<DbHandler>) -> Json<NewDietResponse> {
    let session_uuid = match Uuid::from_str(data.session_id) {
        Ok(r) => r,
        Err(_) => return Json(NewDietResponse::err("invalid session id"))
    };

    let query_user_id = async {
        sqlx::query("SELECT user_id FROM user_session WHERE id = $1")
            .bind(session_uuid)
            .fetch_one(&mut *db)
            .await
    };

    let user_id = match query_user_id.await {
        Ok(r) => r,
        Err(_) => return Json(NewDietResponse::err("failed to query user id"))
    };
    let user_id : i32 = user_id.try_get("user_id").unwrap();

    let query_new_diet = async {
        sqlx::query("INSERT INTO diet(name, user_id) VALUES ($1, $2)")
            .bind(data.diet_name)
            .bind(user_id)
            .execute(&mut *db)
            .await
    };

    match query_new_diet.await {
        Ok(_) => Json(NewDietResponse::ok()),
        Err(_) => Json(NewDietResponse::err("failed to add new diet"))
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
        .attach(DbHandler::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![vue_routes])
        .mount("/api", routes![api_login, api_register, api_logout, api_foods, api_diets, api_delete_diet, api_new_diet])
}
