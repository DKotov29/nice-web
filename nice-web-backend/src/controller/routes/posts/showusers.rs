use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

use crate::model::db::show_user_posts;
use crate::model::session::user::CachedUser;

#[get("/showusers")] // show users POSTS
pub fn showusers(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    cached_user: CachedUser,
) -> (Status, Json<Value>) {
    match pool.get() {
        Ok(conn) => {
            let posts = show_user_posts(conn, cached_user.user.user_id);
            (
                Status::Ok,
                Json(json!({
                    "posts" : posts
                })),
            )
        }
        Err(err) => {
            eprintln!("Pool dont work, check it, error message: {}", err.to_string());
            (
                Status::BadRequest,
                Json(json!({
                    "error": "Unfortunately, server cannot response due to database problem"
                })),
            )
        }
    }
}
