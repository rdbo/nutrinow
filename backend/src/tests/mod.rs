#[cfg(test)]
mod tests {
    use crate::{create_app, create_db, main, settings::Settings};
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };
    use anyhow::Result;

    #[actix_web::test]
    async fn test_start_server() -> Result<()> {
        let settings = Settings::load()?;
        let dbpool = create_db(&settings).await?;
        let app = test::init_service(create_app(dbpool.clone())).await;
        let req = test::TestRequest::get().uri("/").to_request();
        test::call_and_read_body(&app, req).await;
        Ok(())
    }
}
