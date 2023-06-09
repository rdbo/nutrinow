use sqlx::FromRow;
use serde::Serialize;

#[derive(FromRow, Debug, Serialize)]
pub struct Diet {
    pub id : i32,
    pub name : String,
    pub user_id : i32
}

#[derive(FromRow, Debug)]
pub struct Nutrient {
    pub id : i32,
    pub name : String,
    pub unit : String
}

#[derive(FromRow, Debug)]
pub struct DietNutrient {
    pub diet_id : i32,
    pub nutrient_id : i32,
    pub min_intake : f64,
    pub max_intake : f64,
    pub relative : bool
}

#[derive(FromRow, Debug)]
pub struct DietNutrition {
    pub nutrition : Vec<DietNutrient>
}
