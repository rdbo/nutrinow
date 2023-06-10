use actix_web::{post, Responder, web, HttpRequest};
use serde::Deserialize;
use log::info;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::{get_diet_user_id, delete_diet},
        request::get_user_id
    }
};

#[derive(Deserialize, Debug)]
pub struct DeleteDietForm {
    diet_id : i32
}

#[post("/api/delete_diet")]
pub async fn api_delete_diet(form : web::Form<DeleteDietForm>, req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    info!("{:?}", form);
    let mut resp = web::Json(ApiResponse::<&'static str>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let diet_user_id = match get_diet_user_id(form.diet_id, &dbpool).await {
        Some(id) => id,
        None => return web::Json(ApiResponse::<&'static str>::err(ApiError::DeleteDiet)).respond_to(&req)
    };

    if user_id != diet_user_id {
        return web::Json(ApiResponse::<&'static str>::err(ApiError::AccessDenied)).respond_to(&req);
    }

    match delete_diet(form.diet_id, &dbpool).await {
        Ok(_) => web::Json(ApiResponse::ok("OK")).respond_to(&req),
        Err(_) => web::Json(ApiResponse::<&'static str>::err(ApiError::DeleteDiet)).respond_to(&req)
    } 
}
