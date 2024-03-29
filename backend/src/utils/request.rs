use actix_web::{
    cookie::Cookie,
    HttpRequest, HttpResponse
};
use uuid::Uuid;
use std::str::FromStr;
use crate::utils::database::get_session_user_id;
use sqlx::PgPool;

fn create_session_removal_cookie<'a>() -> Cookie<'a> {
    Cookie::build("session_id", "")
        .path("/")
        .finish()
}

pub fn get_session_id<T>(req : &HttpRequest, resp : &mut HttpResponse<T>) -> Option<Uuid> {
    let removal_cookie = create_session_removal_cookie();
    let cookie = match req.cookie("session_id") {
        Some(c) => c,
        None => {
            resp.add_removal_cookie(&removal_cookie).ok();
            return None;
        }
    };

    match Uuid::from_str(cookie.value()) {
        Ok(id) => Some(id),
        Err(_) => {
            resp.add_removal_cookie(&removal_cookie).ok();
            None
        }
    }
}

pub async fn get_user_id<T>(req : &HttpRequest, resp : &mut HttpResponse<T>, dbpool : &PgPool) -> Option<i32> {
    let removal_cookie = create_session_removal_cookie();
    let session_id = get_session_id::<T>(req, resp)?;

    match get_session_user_id(&session_id, dbpool).await {
        None => {
            resp.add_removal_cookie(&removal_cookie).ok();
            None
        },
        some_id => some_id
    }
}
