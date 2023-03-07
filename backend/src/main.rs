use rocket::{
    *,
    fs::{FileServer, relative}
};

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("nutrinow")]
struct DbHandler(sqlx::PgPool);

#[get("/foods")]
async fn api_foods(mut db: Connection<DbHandler>) -> String {
    let rows = sqlx::query("SELECT name, user_id FROM food")
        .fetch_all(&mut *db).await.unwrap();

    let mut result = String::new();
    for row in rows {
        result = format!("{} : {} ; {}", row.get::<i32, usize>(1), row.get::<String, usize>(0), result);
    }

    result
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(DbHandler::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![api_foods])
}
