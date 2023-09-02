use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, Debug)]
pub struct Session{
    token: String
}

#[derive(Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>
}

#[derive(Deserialize, PartialEq)]
pub struct Post {
    pub post_id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub bookmarked: bool
}
