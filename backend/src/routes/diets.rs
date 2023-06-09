use actix_web::{get, Responder, web, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::fetch_user_diets,
        request::get_user_id
    }
};
use cookie::CookieJar;
use uuid::Uuid;
use log::info;

#[get("/api/diets")]
pub async fn api_diets(req : HttpRequest, mut resp : HttpResponse, dbpool : web::Data<PgPool>) -> impl Responder {
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return web::Json(ApiResponse::err(ApiError::NotLoggedIn))
    };

    let diets = match fetch_user_diets(user_id, &dbpool).await {
        Ok(d) => d,
        Err(_) => return web::Json(ApiResponse::err(ApiError::QueryDiets))
    };

    web::Json(ApiResponse::ok(diets))
}
