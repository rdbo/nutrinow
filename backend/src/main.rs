mod settings;
mod routes;
mod models;
mod utils;

use actix_web::{HttpServer, App, get, Result, HttpRequest, middleware::Logger, web};
use actix_files::NamedFile;
use sqlx::postgres::PgPoolOptions;
use std::path::{Path, PathBuf};
use settings::Settings;

#[get("/")]
async fn root() -> Result<NamedFile> {
    let file_path = Path::new("static").join("index.html");
    Ok(NamedFile::open(file_path)?)
}

#[get("/{path:.*}")]
async fn files(req : HttpRequest) -> Result<NamedFile> {
    let path : PathBuf = req.match_info().query("path").parse()?;
    let mut file_path = Path::new("static").join(path);
    if !file_path.exists() {
        file_path = Path::new("static").join("index.html");
    }
    Ok(NamedFile::open(file_path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::load()
        .expect("Failed to load settings. Check your 'Config.toml' or 'ConfigDebug.toml'.");

    let dbpool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&settings.database_url)
        .await
        .expect("Failed to connect to database.");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(dbpool.clone()))
            .service(routes::api_register)
            .service(routes::api_login)
            .service(routes::api_logout)
            .service(routes::api_diets)
            .service(routes::api_diet_nutrition)
            /* Serve index.html on / and on any unmatched routes (necessary to work with Vue.js) */
            .service(root)
            .service(files)
    })
        .bind((settings.host, settings.port))?
        .run()
        .await
}
