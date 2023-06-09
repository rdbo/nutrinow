use actix_web::HttpRequest;
use cookie::{CookieJar, Cookie};
use uuid::Uuid;
use std::str::FromStr;

pub fn get_session_id(req : &HttpRequesr) -> Option<Uuid> {
    let cookie = req.cookie("session_id")?;
    Uuid::from_str(cookie.value()).ok()
}
