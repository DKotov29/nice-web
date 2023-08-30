mod routes;
use rocket::{Build, Rocket};

use crate::controller::routes::signin::signin;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/signup")]
fn signup() -> &'static str {
    "Hello, signup!"
}

#[post("/signout")]
fn signout() -> &'static str {
    "Hello, signup!"
}

#[get("/showusers")]
fn showusers() -> &'static str {
    "Hello, showusers!"
}

pub fn init_pages(server: Rocket<Build>) -> Rocket<Build> {
    let server = server.mount("/", routes![index, signin, signup, showusers]);
    server
}
