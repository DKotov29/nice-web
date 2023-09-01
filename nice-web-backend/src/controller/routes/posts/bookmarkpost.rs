use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::http::Status;
use rocket::State;
use serde_json::json;
use crate::model::session::user::CachedUser;
use crate::model::db::set_bookmark_post;

#[post("/bookmarkpost/<post_id>/<set>")]
pub fn bookmarkpost(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    cached_user: CachedUser,
    post_id: &str,
    set: &str
) -> (Status, String) {
    match pool.get() {
        Ok(conn) => match (post_id.parse::<i32>(), set.parse::<bool>()) {
            (Ok(post_id), Ok(set)) => {
                set_bookmark_post(conn, post_id, cached_user.user.user_id, set);
                (Status::Ok, json!({}).to_string())
            }
            _ => (
                Status::BadRequest,
                json!({
                    "error": "Bad post id or set boolean"
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
