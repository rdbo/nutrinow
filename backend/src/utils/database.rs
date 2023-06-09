use sqlx::{PgPool, Row};
use crate::{
    routes::{
        register::RegisterForm,
        login::LoginForm
    },
    utils::hash::sha256str
};
use uuid::Uuid;
use anyhow::{Error, Result};
use chrono::{Utc, Datelike};

pub async fn create_user_account(data : &RegisterForm, dbpool : &PgPool) -> Result<(), sqlx::Error> {
    let password_hash = sha256str(data.password.as_str());

    let query_result = sqlx::query("INSERT INTO user_account(name, email, gender, weight, birthdate, password_hash) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(&data.name)
        .bind(&data.email)
        .bind(&data.gender)
        .bind(data.weight)
        .bind(&data.birthdate)
        .bind(&password_hash)
        .fetch_one(dbpool)
        .await?;

    let new_user_id : i32 = query_result.try_get("id")?;

    sqlx::query("INSERT INTO credentials(user_id, password) VALUES ($1, $2)")
        .bind(new_user_id)
        .bind(&data.password)
        .execute(dbpool)
        .await
        .ok();

    Ok(())
}

pub async fn authenticate_user(data : &LoginForm, dbpool : &PgPool) -> Result<String> {
    let query_result = sqlx::query("SELECT id, password_hash FROM user_account WHERE email = $1")
        .bind(&data.email)
        .fetch_one(dbpool)
        .await?;

    let user_id : i32 = query_result.try_get("id")?;
    let password_hash : String = query_result.try_get("password_hash")?;

    let attempt_hash = sha256str(data.password.as_str());

    if attempt_hash != password_hash {
        return Err(Error::msg("Passwords don't match"));
    }

    let session_id = Uuid::new_v4();
    let expiry_date = Utc::now();
    let expiry_date = expiry_date.with_year(expiry_date.year() + 1);

    sqlx::query("INSERT INTO user_session(id, user_id, expiry_date) VALUES ($1, $2, $3)")
        .bind(session_id)
        .bind(user_id)
        .bind(expiry_date)
        .execute(dbpool)
        .await?;

    Ok(session_id.to_string())
}
