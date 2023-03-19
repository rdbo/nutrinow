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
