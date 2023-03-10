use rocket::{
    *,
    fs::{FileServer, relative, NamedFile},
    form::Form,
    response::Redirect
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

fn sha256str(string : &str) -> String {
    let mut hash = Sha256::new();
    hash.update(string);
    format!("{:x}", hash.finalize())
}

#[derive(Database)]
#[database("nutrinow")]
struct DbHandler(sqlx::PgPool);

#[get("/foods")]
async fn api_foods(mut db : Connection<DbHandler>) -> String {
    let rows = sqlx::query("SELECT name, user_id FROM food")
        .fetch_all(&mut *db).await.unwrap();

    let mut result = String::new();
    for row in rows {
        result = format!("{} : {} ; {}", row.get::<i32, usize>(1), row.get::<String, usize>(0), result);
    }

    result
}

// Handle Vue routes that are not static files
#[get("/<_..>", rank = 12)]
async fn vue_routes() -> Option<NamedFile> {
    let index_path = PathBuf::from(relative!("static")).join("index.html");
    NamedFile::open(index_path).await.ok()
}

// Register POST
#[derive(FromForm)]
struct RegisterData<'a> {
    name: &'a str,
    birthdate: &'a str,
    email: &'a str,
    password: &'a str,
    gender: &'a str
}

/*
 * TODO: 
 *   Check if e-mail is already registered
 *   Validate user input
 *   Remove unwrap (check for errors)
 */
#[post("/register", data = "<data>")]
async fn register(data : Form<RegisterData<'_>>, mut db : Connection<DbHandler>) -> Redirect {
    let birthdate = NaiveDate::parse_from_str(data.birthdate, "%Y-%m-%d").unwrap();
    let password_hash = sha256str(data.password);
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
        .unwrap();
    Redirect::to(uri!("/login"))
}

// Login POST
#[derive(FromForm)]
struct LoginData<'a> {
    email: &'a str,
    password: &'a str
}

/*
 * TODO:
 *   Remove unwrap (check for errors)
 *   Improve failed login attempt
 */
#[post("/login", data = "<data>")]
async fn login(data : Form<LoginData<'_>>, mut db : Connection<DbHandler>) -> Redirect {
    let result = sqlx::query("SELECT password_hash, id FROM user_account WHERE email = $1")
        .bind(data.email)
        .fetch_one(&mut *db).await.unwrap();

    let password_hash = result.try_get::<&str, usize>(0).unwrap();
    let attempt_hash = sha256str(data.password);
    if attempt_hash != password_hash {
        return Redirect::to(uri!("/login"));
    }

    let user_id = result.try_get::<i32, usize>(1).unwrap();
    let session_id = Uuid::new_v4();
    let expiry_date = Utc::now();
    let expiry_date = expiry_date.with_year(expiry_date.year() + 1);
    sqlx::query("INSERT INTO user_session(id, user_id, expiry_date) VALUES ($1, $2, $3)")
        .bind(session_id)
        .bind(user_id)
        .bind(expiry_date)
        .execute(&mut *db).await.unwrap();

    Redirect::to(uri!("/"))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(DbHandler::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![vue_routes, register, login])
        .mount("/api", routes![api_foods])
}
