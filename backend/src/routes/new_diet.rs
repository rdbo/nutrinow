use actix_web::{post, Responder, web, HttpRequest};
use serde::Deserialize;
use log::info;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::create_diet,
        request::get_user_id
    }
};

#[derive(Deserialize, Debug)]
pub struct NewDietForm {
    diet_name : String
}

#[post("/api/new_diet")]
pub async fn api_new_diet(form : web::Form<NewDietForm>, req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    info!("{:?}", form);
    let mut resp = web::Json(ApiResponse::<&'static str>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    match create_diet(user_id, &form.diet_name, &dbpool).await {
        Some(_) => web::Json(ApiResponse::ok("OK")).respond_to(&req),
        None => web::Json(ApiResponse::<&'static str>::err(ApiError::CreateDiet)).respond_to(&req)
    } 
}
