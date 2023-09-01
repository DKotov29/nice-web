use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::http::Status;
use rocket::State;
use serde::Serializer;
use serde_json::json;
use crate::model::db::show_user_posts;
use crate::model::session::user::CachedUser;

#[post("/showusers")] // show users POSTS
pub fn showusers(pool: &State<Pool<ConnectionManager<PgConnection>>>, cached_user: CachedUser) -> (Status, String) {
    match pool.get() {
        Ok(conn) => {
            let posts = show_user_posts(conn, cached_user.user.user_id);
            (Status::Ok, json!({
                "posts" : posts
            }).to_string()) // todo: check is needed, its possible that this will not be the same as in api.md
        }
        Err(err) => {
            println!("Pool dont work, check it, error message: {}", err.to_string()); // todo some logging?
            (
                Status::BadRequest,
                json!({
                    "error": "Unfortunately, server cannot response due to database problem"
                })
                    .to_string(),
            )
        }
    }
}
