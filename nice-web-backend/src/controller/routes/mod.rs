use rocket::serde::Deserialize;

pub mod posts;
pub mod signin;
pub mod signout;
pub mod signup;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    username: String,
    password: String,
}
