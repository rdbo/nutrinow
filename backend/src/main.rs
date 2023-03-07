use rocket::{
    *,
    fs::{FileServer, relative}
};

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("nutrinow")]
struct StoredData(sqlx::PgPool);

#[get("/foods")]
async fn api_foods(mut db: Connection<Logs>) -> String {
    sqlx::query("SELECT name FROM food")
        .fetch_all(&mut *db).await?
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(StoredData::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![api_foods])
}
