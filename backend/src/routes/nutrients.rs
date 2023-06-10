use actix_web::{get, Responder, web};
use serde::Serialize;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError, Nutrient},
    utils::database::fetch_nutrients
};

#[derive(Serialize, Debug)]
struct NutrientsResponse {
    nutrients : Vec<Nutrient>
}

#[get("/api/nutrients")]
pub async fn api_nutrients(dbpool : web::Data<PgPool>) -> impl Responder {
    let nutrients = match fetch_nutrients(&dbpool).await {
        Some(nutrients) => nutrients,
        None => return web::Json(ApiResponse::err(ApiError::QueryNutrients))
    };

    web::Json(ApiResponse::ok(NutrientsResponse { nutrients }))
}
