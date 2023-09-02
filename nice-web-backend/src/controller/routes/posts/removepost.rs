use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

use crate::model::db::delete_post;
use crate::model::session::user::CachedUser;
use crate::{ConnectionManager, PgConnection, Pool};

#[delete("/removepost/<post_id>")]
pub fn removepost(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    cached_user: CachedUser,
    post_id: &str,
) -> (Status, Json<Value>) {
    match pool.get() {
        Ok(conn) => match post_id.parse::<i32>() {
            Ok(post_id) => {
                delete_post(conn, post_id, cached_user.user.user_id);
                (Status::Ok, Json(json!({})))
            }
            Err(_err) => (
                Status::BadRequest,
                Json(json!({
                    "error": "Bad post id"
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
