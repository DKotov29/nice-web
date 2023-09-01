use std::num::ParseIntError;

use rocket::http::Status;
use rocket::State;
use serde_json::json;

use crate::model::db::delete_post;
use crate::model::session::user::CachedUser;
use crate::{ConnectionManager, PgConnection, Pool};

#[post("/removepost/<post_id>")]
pub fn removepost(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    cached_user: CachedUser,
    post_id: &str,
) -> (Status, String) {
    match pool.get() {
        Ok(conn) => match post_id.parse::<i32>() {
            Ok(post_id) => {
                delete_post(conn, post_id, cached_user.user.user_id);
                (Status::Ok, json!({}).to_string())
            }
            Err(err) => (
                Status::BadRequest,
                json!({
                    "error": "Bad post id"
                })
                .to_string(),
            ),
        },
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
