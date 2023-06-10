use actix_web::{get, Responder, web, HttpRequest};
use serde::Serialize;
use sqlx::{PgPool, FromRow};
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::{get_diet_user_id, fetch_diet_meals, fetch_meal_info_foods},
        request::get_user_id
    }
};

#[derive(Serialize, FromRow, Debug)]
pub struct MealInfoNutrient {
    pub name : String,
    pub amount : f64,
    pub unit : String
}

#[derive(Serialize, Debug)]
pub struct MealInfoFood {
    pub id : i32,
    pub name : String,
    pub meal_serving_id : i32,
    pub serving_id : i32,
    pub serving_base : f64,
    pub serving_amount : f64,
    pub serving_unit : String,
    pub base_nutrients : Vec<MealInfoNutrient>
}

#[derive(Serialize, Debug)]
pub struct MealInfo {
    pub id : i32,
    pub name : String,
    pub foods : Vec<MealInfoFood>
}

#[derive(Serialize, Debug)]
struct MealsResponse {
    meals : Vec<MealInfo>
}

#[get("/api/meals/{diet_id}")]
pub async fn api_meals(diet_id : web::Path<i32>, req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    let diet_id = diet_id.into_inner();

    let mut resp = web::Json(ApiResponse::<MealsResponse>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let diet_user_id = match get_diet_user_id(diet_id, &dbpool).await {
        Some(id) => id,
        None => return web::Json(ApiResponse::<MealsResponse>::err(ApiError::QueryMeals)).respond_to(&req)
    };

    if user_id != diet_user_id {
        return web::Json(ApiResponse::<MealsResponse>::err(ApiError::AccessDenied)).respond_to(&req);
    }

    let meals = match fetch_diet_meals(diet_id, &dbpool).await {
        Some(meals) => meals,
        None => return web::Json(ApiResponse::<MealsResponse>::err(ApiError::QueryMeals)).respond_to(&req)
    };

    let mut meals_info : Vec<MealInfo> = vec![];
    for meal in &meals {
        let meal_info_foods = match fetch_meal_info_foods(meal.id, &dbpool).await {
            Some(foods) => foods,
            None => return web::Json(ApiResponse::<MealsResponse>::err(ApiError::QueryMeals)).respond_to(&req)
        };

        meals_info.push(MealInfo { id: meal.id, name: meal.name.clone(), foods: meal_info_foods });
    }

    web::Json(ApiResponse::ok(MealsResponse { meals: meals_info })).respond_to(&req)
}
