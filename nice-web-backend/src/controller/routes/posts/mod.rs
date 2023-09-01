use rocket::serde::Deserialize;

pub mod showusers;
pub mod createpost;
pub mod removepost;
pub mod bookmarkpost;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub title: String,
    pub description: String
}
