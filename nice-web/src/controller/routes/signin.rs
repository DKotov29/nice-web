use rocket::serde::{Deserialize, json::Json};
use rocket::State;
use crate::controller::routes::Credentials;

#[post("/signin",  data = "<credentials>")]
pub fn signin(credentials: Json<Credentials>) -> String { //  pool: &State<>
    // let pool =
    format!("{:?}", credentials)
}
