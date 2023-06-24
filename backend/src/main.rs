use actix_web::{middleware::Logger, HttpServer};
use nutrinow::{connect_db, create_app, settings::Settings};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::load()
        .expect("Failed to load settings. Check your 'Config.toml' or 'ConfigDebug.toml'.");

    let dbpool = connect_db(&settings)
        .await
        .expect("Failed to connect to database.");

    env_logger::init();

    HttpServer::new(move || create_app(dbpool.clone()).wrap(Logger::default()))
        .bind((settings.host, settings.port))?
        .run()
        .await
}
