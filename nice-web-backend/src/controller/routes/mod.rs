use rocket::serde::Deserialize;

pub mod signin;
pub mod signup;
pub mod signout;
pub mod posts;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    username: String,
    password: String,
}
