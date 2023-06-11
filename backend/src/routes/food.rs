use actix_web::{get, Responder, web};
use serde::Serialize;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::database::{fetch_search_food},
    routes::food_search::SearchFood
};

#[derive(Serialize, Debug)]
pub struct FoodResponse {
    food : SearchFood
}

#[get("/api/food/{food_id}")]
pub async fn api_food(food_id : web::Path<i32>, dbpool : web::Data<PgPool>) -> impl Responder {
    let food_id = food_id.into_inner();

    match fetch_search_food(food_id, &dbpool).await {
        Some(food) => web::Json(ApiResponse::ok(FoodResponse { food })),
        None => web::Json(ApiResponse::err(ApiError::QueryFood))
    }
}
