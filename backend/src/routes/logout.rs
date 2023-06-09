use actix_web::{
    http::StatusCode,
    post, Responder, web, HttpRequest, HttpResponse
};
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        request::get_session_id,
        database::delete_session
    }
};

#[post("/api/logout")]
pub async fn api_logout(req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    let session_id = match get_session_id(&req, &mut HttpResponse::new(StatusCode::OK)) {
        Some(session_id) => session_id,
        None => return web::Json(ApiResponse::err(ApiError::NotLoggedIn))
    };

    delete_session(session_id, &dbpool).await.ok();

    web::Json(ApiResponse::ok("OK"))
}
