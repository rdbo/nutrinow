use actix_web::{get, Responder, web, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError, Diet},
    utils::{
        database::fetch_user_diets,
        request::get_user_id
    }
};
use cookie::CookieJar;
use uuid::Uuid;
use log::info;

#[get("/api/diets")]
pub async fn api_diets(req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    let mut resp = web::Json(ApiResponse::<Vec<Diet>>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let diets = match fetch_user_diets(user_id, &dbpool).await {
        Ok(d) => d,
        Err(_) => return web::Json(ApiResponse::<Vec<Diet>>::err(ApiError::QueryDiets)).respond_to(&req)
    };

    web::Json(ApiResponse::ok(diets)).respond_to(&req)
}
