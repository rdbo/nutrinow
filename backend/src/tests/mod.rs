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
        info!("Initializing actix-web server");
        let settings = Settings::load()?;
        info!("Settings: {:?}", settings);
        let dbpool = create_db(&settings).await?;
        let app = test::init_service(create_app(dbpool.clone())).await;
        let req = test::TestRequest::get().uri("/").to_request();
        info!("Sending GET request to /");
        let resp = test::call_service(&app, req).await;
        info!("Response: {}", resp.status());
        assert!(resp.status().is_success());
        Ok(())
    }
}
