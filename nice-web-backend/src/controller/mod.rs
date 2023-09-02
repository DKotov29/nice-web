mod routes;

use rocket::serde::json::Json;
use serde_json::{json, Value};
use rocket::{Build, Rocket};

use crate::controller::routes::signin::signin;
use crate::controller::routes::signup::signup;
use crate::controller::routes::signout::signout;
use crate::controller::routes::posts::showusers::showusers;
use crate::controller::routes::posts::createpost::createpost;
use crate::controller::routes::posts::removepost::removepost;
use crate::controller::routes::posts::bookmarkpost::bookmarkpost;

#[catch(500)]
fn internal_error() -> Json<Value> {
    Json(json!({
        "error": "Something went wrong"
    }))
}

pub fn init_pages(server: Rocket<Build>) -> Rocket<Build> {
    server.mount("/", routes![signin, signout, signup, showusers, createpost, removepost, bookmarkpost])
        .register("/", catchers![internal_error])
}
