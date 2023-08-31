use rocket::serde::Deserialize;

pub mod signin;
pub mod signup;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    username: String,
    password: String
}