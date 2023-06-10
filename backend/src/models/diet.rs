use sqlx::FromRow;
use serde::Serialize;

#[derive(FromRow, Serialize, Debug)]
pub struct Diet {
    pub id : i32,
    pub name : String,
    pub user_id : i32
}

#[derive(FromRow, Serialize, Debug)]
pub struct Nutrient {
    pub id : i32,
    pub name : String,
    pub unit : String
}

#[derive(FromRow, Serialize, Debug)]
pub struct DietNutrient {
    pub diet_id : i32,
    pub nutrient_id : i32,
    pub min_intake : Option<f64>,
    pub max_intake : Option<f64>,
    pub relative : bool
}

#[derive(FromRow, Serialize, Debug)]
pub struct DietNutrition {
    pub nutrition : Vec<DietNutrient>
}
