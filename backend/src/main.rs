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

// Food Request

/*
 * TODO:
 *   Add sorting methods
 *   Limit results
 */

#[derive(Serialize)]
struct FoodInfo {
    name : String,
    cals: f32,
    carbo: f32,
    proteins: f32,
    fats: f32
}

#[get("/foods")]
async fn api_foods(mut db : Connection<DbHandler>) -> Json {
    /*
     * SELECT * FROM food JOIN serving ON serving.food_id = food.id JOIN serving_nutrient ON serving_nutrient.serving_id = serving.id JOIN nutrient ON nutrient.id = serving_nutrient.nutrient_id WHERE nutrient.name IN ('Protein', 'Carbohydrates', 'Fats');
     */
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
        .mount("/api", routes![api_login, api_register, api_logout])
}
