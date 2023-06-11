use actix_web::{post, Responder, web, HttpRequest};
use serde::Deserialize;
use log::info;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::{get_meal_user_id, add_meal_serving},
        request::get_user_id
    }
};

#[derive(Deserialize, Debug)]
pub struct AddMealServingForm {
    meal_id : i32,
    serving_id : i32,
    amount : f64
}

#[post("/api/add_meal_serving")]
pub async fn api_add_meal_serving(form : web::Form<AddMealServingForm>, req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    info!("{:?}", form);
    let mut resp = web::Json(ApiResponse::<&'static str>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let meal_user_id = match get_meal_user_id(form.meal_id, &dbpool).await {
        Some(id) => id,
        None => return web::Json(ApiResponse::<&'static str>::err(ApiError::AddMealServing)).respond_to(&req)
    };

    if user_id != meal_user_id {
        return web::Json(ApiResponse::<&'static str>::err(ApiError::AccessDenied)).respond_to(&req);
    }

    match add_meal_serving(form.meal_id, form.serving_id, form.amount, &dbpool).await {
        Ok(_) => web::Json(ApiResponse::ok("OK")).respond_to(&req),
        Err(_) => web::Json(ApiResponse::<&'static str>::err(ApiError::AddMealServing)).respond_to(&req)
    } 
}
