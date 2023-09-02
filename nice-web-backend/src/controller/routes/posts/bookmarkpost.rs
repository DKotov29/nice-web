use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

use crate::model::db::set_bookmark_post;
use crate::model::session::user::CachedUser;

#[post("/bookmarkpost/<post_id>/<set>")]
pub fn bookmarkpost(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    cached_user: CachedUser,
    post_id: &str,
    set: &str,
) -> (Status, Json<Value>) {
    match pool.get() {
        Ok(conn) => match (post_id.parse::<i32>(), set.parse::<bool>()) {
            (Ok(post_id), Ok(set)) => {
                set_bookmark_post(conn, post_id, cached_user.user.user_id, set);
                (Status::Ok, Json(json!({})))
            }
            _ => (
                Status::BadRequest,
                Json(json!({
                    "error": "Bad post id or set boolean"
                })),
            ),
        },
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
