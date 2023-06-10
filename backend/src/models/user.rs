use sqlx::FromRow;
use serde::Serialize;
use chrono::NaiveDate;

#[derive(FromRow, Serialize, Debug)]
pub struct UserAccount {
    pub id : i32,
    pub name : String,
    pub email : String,
    pub gender : String,
    pub weight : f64,
    pub birthdate : NaiveDate
}
