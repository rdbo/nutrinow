use actix_web::{get, Responder, web, HttpRequest};
use serde::Serialize;
use sqlx::PgPool;
use crate::{
    models::{ApiResponse, ApiError, Diet},
    utils::{
        database::fetch_user_diets,
        request::get_user_id
    }
};

#[derive(Serialize, Debug)]
struct DietInfoNutrient {
    name : String,
    min_amount : Option<f64>,
    max_amount : Option<f64>,
    unit : String,
    relative : bool
}

#[derive(Serialize, Debug)]
struct DietInfo {
    id : i32,
    name : String,
    desired_nutrition : Vec<DietInfoNutrient>
}

#[derive(Serialize, Debug)]
struct DietsResponse {
    diets : Vec<DietInfo>
}

#[get("/api/diets")]
pub async fn api_diets(req : HttpRequest, dbpool : web::Data<PgPool>) -> impl Responder {
    let mut resp = web::Json(ApiResponse::<Vec<Diet>>::err(ApiError::NotLoggedIn)).respond_to(&req);
    let user_id = match get_user_id(&req, &mut resp, &dbpool).await {
        Some(id) => id,
        None => return resp
    };

    let diets = match fetch_user_diets(user_id, &dbpool).await {
        Ok(d) => d,
        Err(_) => return web::Json(ApiResponse::<Vec<Diet>>::err(ApiError::QueryDiets)).respond_to(&req)
    };
    
    let diets_info : Vec<DietInfo> = diets
        .into_iter()
        .map(|d| { DietInfo { id: d.id, name: d.name, desired_nutrition: vec![] } })
        .collect();

    web::Json(ApiResponse::ok(DietsResponse { diets: diets_info })).respond_to(&req)
}
