use actix_web::{get, Responder, web};
use serde::Serialize;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::database::search_foods,
    routes::meals::MealInfoNutrient
};

#[derive(Serialize, Debug)]
pub struct SearchFoodServing {
    pub id : i32,
    pub amount : f64,
    pub unit : String,
    pub nutrients : Vec<MealInfoNutrient>,
    pub relative : Option<i32>
}

#[derive(Serialize, Debug)]
pub struct SearchFood {
    pub id : i32,
    pub name : String,
    pub servings : Vec<SearchFoodServing>
}

#[derive(Serialize, Debug)]
pub struct FoodSearchResponse {
    matches : Vec<SearchFood>
}

#[get("/api/food_search/{food_name}")]
pub async fn api_food_search(food_name : web::Path<String>, dbpool : web::Data<PgPool>) -> impl Responder {
    let food_name = food_name.into_inner();

    match search_foods(&food_name, &dbpool).await {
        Some(matches) => web::Json(ApiResponse::ok(FoodSearchResponse { matches })),
        None => web::Json(ApiResponse::err(ApiError::SearchFoods))
    }
}
