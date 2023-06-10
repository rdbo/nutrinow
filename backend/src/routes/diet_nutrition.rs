use actix_web::{get, Responder, web, HttpRequest};
use serde::Serialize;
use sqlx::{PgPool, FromRow};
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::{get_diet_user_id, fetch_diet_info_nutrition},
        request::get_user_id
    }
};

#[derive(Serialize, FromRow, Debug)]
pub struct DietInfoNutrient {
    name : String,
    min_amount : Option<f64>,
    max_amount : Option<f64>,
    unit : String,
    relative : bool
}

#[derive(Serialize, Debug)]
struct DietNutritionResponse {
    nutrition : Vec<DietInfoNutrient>
}

#[get("/api/diet_nutrition/{diet_id}")]
pub async fn api_diet_nutrition(diet_id : web::Path<i32>, req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    let diet_id = diet_id.into_inner();

    let mut resp = web::Json(ApiResponse::<DietNutritionResponse>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let diet_user_id = match get_diet_user_id(diet_id, &dbpool).await {
        Some(id) => id,
        None => return web::Json(ApiResponse::<DietNutritionResponse>::err(ApiError::QueryDietNutrition)).respond_to(&req)
    };

    if user_id != diet_user_id {
        return web::Json(ApiResponse::<DietNutritionResponse>::err(ApiError::AccessDenied)).respond_to(&req);
    }

    let diet_info_nutrients = match fetch_diet_info_nutrition(diet_id, &dbpool).await {
        Some(nutrients) => nutrients,
        None => return web::Json(ApiResponse::<DietNutritionResponse>::err(ApiError::QueryDietNutrition)).respond_to(&req)
    };

    web::Json(ApiResponse::ok(DietNutritionResponse { nutrition: diet_info_nutrients })).respond_to(&req)
}
