use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/signin")]
fn signin() -> &'static str {
    "Hello, signin!"
}

#[get("/signup")]
fn signup() -> &'static str {
    "Hello, signup!"
}

#[get("/signout")]
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


