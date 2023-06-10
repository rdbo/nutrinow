use actix_web::{get, Responder, web, HttpRequest};
use serde::Serialize;
use sqlx::PgPool;
use chrono::NaiveDate;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::fetch_user_account,
        request::get_user_id
    }
};

#[derive(Serialize, Debug)]
struct UserResponse {
    name : String,
    birthdate : NaiveDate,
    gender : String,
    weight : f64
}

#[get("/api/user")]
pub async fn api_user(req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    let mut resp = web::Json(ApiResponse::<UserResponse>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let user_account = match fetch_user_account(user_id, &dbpool).await {
        Some(user) => user,
        None => return web::Json(ApiResponse::<UserResponse>::err(ApiError::QueryDiets)).respond_to(&req)
    };

    let user_resp = UserResponse {
        name: user_account.name.clone(),
        birthdate: user_account.birthdate,
        gender: user_account.gender,
        weight: user_account.weight
    };

    web::Json(ApiResponse::ok(user_resp)).respond_to(&req)
}
