use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

use crate::controller::routes::posts::Post;
use crate::model::db::create_user_post;
use crate::model::session::user::CachedUser;

#[post("/createpost", data = "<post_data>")]
pub fn createpost(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    cached_user: CachedUser,
    post_data: Json<Post>,
) -> (Status, Json<Value>) {
    match pool.get() {
        Ok(conn) => {
            create_user_post(
                conn,
                cached_user.user.user_id,
                post_data.title.clone(),
                post_data.description.clone(),
            );
            (Status::Ok, Json(json!({})))
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
