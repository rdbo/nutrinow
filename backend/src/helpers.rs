use rocket::{
    http::{CookieJar, Cookie}
};

use rocket_db_pools::{
    sqlx::{
        self, Row, Postgres,
        pool::PoolConnection,
        types::uuid::Uuid
    }
};

use sha2::{Sha256, Digest};

use std::str::FromStr;

pub fn sha256str(string : &str) -> String {
    let mut hash = Sha256::new();
    hash.update(string);
    format!("{:x}", hash.finalize())
}

use chrono::{
    Utc, NaiveDate
};

pub fn session_id_from_cookies(cookies : &CookieJar<'_>) -> Result<Uuid, &'static str> {
    let session_uuid = match cookies.get("session_id") {
        Some(id) => id,
        None => return Err("Missing 'session_id' cookie")
    };

    let session_id = match Uuid::from_str(session_uuid.value()) {
        Ok(r) => r,
        _ => return Err("The session ID is invalid")
    };

    Ok(session_id)
}

pub async fn user_id_from_cookies(cookies : &CookieJar<'_>, db : &mut PoolConnection<Postgres>) -> Result<i32, &'static str> {
    let session_id = match session_id_from_cookies(cookies) {
        Ok(id) => id,
        Err(s) => return Err(s)
    };

    let query_user_id = async {
        sqlx::query("SELECT user_id FROM user_session WHERE id = $1")
            .bind(session_id)
            .fetch_one(db)
            .await
    };
    let user_id = match query_user_id.await {
        Ok(r) => r,
        Err(_) => {
            cookies.remove(Cookie::named("session_id"));
            return Err("Failed to query user ID")
        }
    };
    let user_id : i32 = user_id.try_get("user_id").unwrap();

    Ok(user_id)
}

pub async fn diet_owner_id(diet_id : i32, db : &mut PoolConnection<Postgres>) -> Result<i32, &'static str> {
    let query_diet_owner_id = async {
        sqlx::query("SELECT user_id FROM diet WHERE id = $1")
            .bind(diet_id)
            .fetch_one(&mut *db)
            .await
    };

    let diet_owner_id = match query_diet_owner_id.await {
        Ok(r) => r,
        Err(_) => return Err("Failed to query diet owner id")
    };

    let diet_owner_id : i32 = diet_owner_id.try_get("user_id").unwrap();

    Ok(diet_owner_id)
}

pub async fn meal_owner_id(meal_id : i32, db : &mut PoolConnection<Postgres>) -> Result<i32, &'static str> {
    let query_diet_owner_id = async {
        sqlx::query("SELECT user_id FROM meal JOIN diet ON diet.id = meal.diet_id WHERE meal.id = $1")
            .bind(meal_id)
            .fetch_one(&mut *db)
            .await
    };

    let diet_owner_id = match query_diet_owner_id.await {
        Ok(r) => r,
        Err(_) => return Err("Failed to query diet owner id")
    };
    let diet_owner_id : i32 = diet_owner_id.try_get("user_id").unwrap();

    Ok(diet_owner_id)
}

pub struct UserInformation {
    pub weight : f64,
    pub gender : String,
    pub birthdate : NaiveDate
}

pub async fn user_information(user_id : i32, db : &mut PoolConnection<Postgres>) -> Result<UserInformation, &'static str> {
    let query_user_info = async {
        sqlx::query("SELECT gender, weight, birthdate FROM user_account WHERE id = $1")
            .bind(user_id)
            .fetch_one(&mut *db)
            .await
    };

    let user_info = match query_user_info.await {
        Ok(r) => r,
        Err(_) => return Err("Failed to query user information")
    };

    let user_information = UserInformation {
        gender: user_info.try_get("gender").unwrap(),
        weight: user_info.try_get("weight").unwrap(),
        birthdate: user_info.try_get("birthdate").unwrap()
    };

    Ok(user_information)
}

pub fn calculate_age(date : &NaiveDate) -> u32 {
    let now = Utc::now().date_naive();
    match now.years_since(*date) {
        Some(y) => y,
        None => 0
    }
}
