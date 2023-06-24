mod models;
mod routes;
mod settings;
mod tests;
mod utils;

use actix_files::NamedFile;
use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    get,
    middleware::Logger,
    web, App, HttpRequest, HttpServer, Result,
};
use settings::Settings;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::path::{Path, PathBuf};

#[get("/")]
async fn root() -> Result<NamedFile> {
    let file_path = Path::new("static").join("index.html");
    Ok(NamedFile::open(file_path)?)
}

#[get("/{path:.*}")]
async fn files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("path").parse()?;
    let mut file_path = Path::new("static").join(path);
    if !file_path.exists() {
        file_path = Path::new("static").join("index.html");
    }
    Ok(NamedFile::open(file_path)?)
}

pub async fn create_db(settings: &Settings) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&settings.database_url)
        .await
}

pub fn create_app(
    dbpool: Pool<Postgres>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(dbpool))
        .service(routes::api_register)
        .service(routes::api_login)
        .service(routes::api_logout)
        .service(routes::api_diets)
        .service(routes::api_diet_nutrition)
        .service(routes::api_nutrients)
        .service(routes::api_meals)
        .service(routes::api_user)
        .service(routes::api_new_diet)
        .service(routes::api_edit_diet)
        .service(routes::api_delete_diet)
        .service(routes::api_add_meal)
        .service(routes::api_delete_meal)
        .service(routes::api_food_search)
        .service(routes::api_add_meal_serving)
        .service(routes::api_delete_meal_serving)
        .service(routes::api_edit_meal_serving)
        .service(routes::api_food)
        .service(routes::api_duplicate_diet)
        /* Serve index.html on / and on any unmatched routes (necessary to work with Vue.js) */
        .service(root)
        .service(files)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::load()
        .expect("Failed to load settings. Check your 'Config.toml' or 'ConfigDebug.toml'.");

    let dbpool = create_db(&settings)
        .await
        .expect("Failed to connect to database.");

    env_logger::init();

    HttpServer::new(move || create_app(dbpool.clone()).wrap(Logger::default()))
        .bind((settings.host, settings.port))?
        .run()
        .await
}
