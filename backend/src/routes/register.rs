use actix_web::{post, Responder, web};
use serde::{Deserialize};
use log::info;
use chrono::NaiveDate;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::database::create_user_account
};

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub name : String,
    pub birthdate : NaiveDate,
    pub email : String,
    pub password : String,
    pub gender : char,
    pub weight : f64
}

#[post("/api/register")]
pub async fn api_register(form : web::Form<RegisterForm>, dbpool : web::Data<PgPool>) -> impl Responder {
    info!("{:?}", form);
    match create_user_account(&form, &dbpool).await {
        Ok(_) => web::Json(ApiResponse::ok("OK")),
        Err(_) => web::Json(ApiResponse::err(ApiError::RegistrationFailed))
    }
}