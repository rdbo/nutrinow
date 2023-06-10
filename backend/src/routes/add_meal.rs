use actix_web::{post, Responder, web, HttpRequest};
use serde::{Serialize, Deserialize};
use log::info;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError},
    utils::{
        database::{get_diet_user_id, create_meal},
        request::get_user_id
    },
    routes::meals::MealInfo
};

#[derive(Deserialize, Debug)]
pub struct AddMealForm {
    diet_id : i32,
    meal_name : String
}

#[derive(Serialize, Debug)]
struct AddMealResponse {
    pub meal : MealInfo
}

#[post("/api/add_meal")]
pub async fn api_add_meal(form : web::Form<AddMealForm>, req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    info!("{:?}", form);
    let mut resp = web::Json(ApiResponse::<AddMealResponse>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let diet_user_id = match get_diet_user_id(form.diet_id, &dbpool).await {
        Some(id) => id,
        None => return web::Json(ApiResponse::<&'static str>::err(ApiError::CreateMeal)).respond_to(&req)
    };

    if user_id != diet_user_id {
        return web::Json(ApiResponse::<&'static str>::err(ApiError::AccessDenied)).respond_to(&req);
    }

    match create_meal(form.diet_id, &form.meal_name, &dbpool).await {
        Some(id) => {
            let meal = MealInfo { id, name: form.meal_name.clone(), foods: vec![] };
            web::Json(ApiResponse::ok(AddMealResponse { meal })).respond_to(&req)
        }
        None => web::Json(ApiResponse::<AddMealResponse>::err(ApiError::CreateMeal)).respond_to(&req)
    } 
}
