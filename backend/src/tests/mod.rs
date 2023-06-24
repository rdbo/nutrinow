#[cfg(test)]
mod tests {
    use crate::{create_app, create_db, settings::Settings};
    use actix_web::test;
    use anyhow::Result;
    use env_logger;
    use log::info;

    #[actix_web::test]
    async fn test_start_server() -> Result<()> {
        env_logger::init();
        info!("Loading settings");
        let settings = Settings::load()?;
        info!("Settings: {:?}", settings);
        info!("Starting database");
        let dbpool = create_db(&settings).await?;
        info!("Starting web server");
        let app = test::init_service(create_app(dbpool.clone())).await;
        info!("Sending GET request to /");
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        info!("Response: {}", resp.status());
        assert!(resp.status().is_success());
        Ok(())
    }
}
