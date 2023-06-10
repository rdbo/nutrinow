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

#[derive(FromRow, Serialize, Debug)]
pub struct Food {
    pub id : i32,
    pub name : String,
    pub user_id : i32
}

#[derive(FromRow, Serialize, Debug)]
pub struct Serving {
    pub id : i32,
    pub food_id : i32,
    pub unit : String,
    pub amount : f64,
    pub relative : Option<i32>
}

#[derive(FromRow, Serialize, Debug)]
pub struct Meal {
    pub id : i32,
    pub diet_id : i32,
    pub name : String
}

#[derive(FromRow, Serialize, Debug)]
pub struct MealServing {
    pub id : i32,
    pub meal_id : i32,
    pub serving_id : i32,
    pub amount : f64
}

#[derive(FromRow, Serialize, Debug)]
pub struct DefaultNutrient {
    pub nutrient_id : i32,
    pub min_intake : Option<f64>,
    pub max_intake : Option<f64>,
    pub relative : bool,
    pub gender : String,
    pub age_min : i32,
    pub age_max : Option<i32>
}
