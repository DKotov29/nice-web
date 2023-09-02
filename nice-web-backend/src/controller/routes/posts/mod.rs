use rocket::serde::Deserialize;

pub mod bookmarkpost;
pub mod createpost;
pub mod removepost;
pub mod showusers;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub title: String,
    pub description: String,
}
