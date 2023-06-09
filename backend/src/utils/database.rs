use sqlx::{PgPool, Error, Row};
use crate::{
    routes::register::RegisterForm,
    utils::hash::sha256str
};

pub async fn create_user_account(dbpool : &PgPool, data : &RegisterForm) -> Result<(), Error> {
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
