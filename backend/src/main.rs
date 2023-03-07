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
    sqlx::query("SELECT name FROM food")
        .fetch_one(&mut *db).await.unwrap().try_get(0).unwrap()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(DbHandler::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![api_foods])
}
