use rocket::serde::json::Json;
use crate::controller::routes::Credentials;

// check if password matches pattern
#[post("/signup", data = "<credentials>")]
pub fn signup(credentials: Json<Credentials>) -> String {
    format!("{:?}", credentials)
}
