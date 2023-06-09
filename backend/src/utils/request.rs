use actix_web::{
    cookie::{CookieJar, Cookie},
    HttpRequest, HttpResponse
};
use uuid::Uuid;
use std::str::FromStr;
use crate::utils::database::get_session_user_id;
use sqlx::PgPool;

pub fn get_session_id(req : &HttpRequest, resp : &mut HttpResponse) -> Option<Uuid> {
    let removal_cookie = Cookie::named("session_id");
    let cookie = match req.cookie("session_id") {
        Some(c) => c,
        None => {
            resp.add_removal_cookie(&removal_cookie);
            return None;
        }
    };

    match Uuid::from_str(cookie.value()) {
        Ok(id) => Some(id),
        Err(_) => {
            resp.add_removal_cookie(&removal_cookie);
            None
        }
    }
}

pub async fn get_user_id(req : &HttpRequest, resp : &mut HttpResponse, dbpool : &PgPool) -> Option<i32> {
    let removal_cookie = Cookie::named("session_id");
    let session_id = get_session_id(req, resp)?;

    match get_session_user_id(&session_id, dbpool).await {
        None => {
            resp.add_removal_cookie(&removal_cookie);
            None
        },
        some_id => some_id
    }
}
