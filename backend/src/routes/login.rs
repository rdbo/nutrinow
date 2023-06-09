use actix_web::{post, Responder, web};
use serde::{Serialize, Deserialize};
use log::info;
use sqlx::PgPool;
use crate::{
    models::JsonResponse,
    utils::database::authenticate_user
};

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub email : String,
    pub password : String
}

#[derive(Serialize)]
struct LoginResponse {
    pub session_id : String
}

#[derive(Serialize)]
enum Response {
    Success(LoginResponse),
    Failure(JsonResponse)
}

#[post("/api/login")]
pub async fn api_login(form : web::Form<LoginForm>, dbpool : web::Data<PgPool>) -> impl Responder {
    info!("{:?}", form);
    let session_id = match authenticate_user(&form, &dbpool).await {
        Ok(session_id) => session_id,
        Err(_) => return web::Json(Response::Failure(JsonResponse::err("Login attempt failed".to_string())))
    };

    web::Json(Response::Success(LoginResponse { session_id }))
}
