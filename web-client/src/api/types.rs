use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, Debug)]
pub struct Session(String);

#[derive(Serialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>
}

#[derive(Deserialize)]
pub struct Post {
    pub post_id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub bookmarked: bool
}
