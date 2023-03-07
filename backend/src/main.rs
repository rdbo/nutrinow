use rocket::{
    *,
    fs::{FileServer, relative}
};

#[get("/")]
async fn index() -> &'static str {
    "Hello World"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("static")))
}
