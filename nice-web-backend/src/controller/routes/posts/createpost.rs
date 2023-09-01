use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::http::Status;
use rocket::State;
use crate::controller::routes::posts::Post;
use crate::model::session::user::CachedUser;
use rocket::serde::json::Json;
use serde_json::json;
use crate::model::db::create_user_post;

#[post("/createpost", data = "<post_data>")]
pub fn createpost(pool: &State<Pool<ConnectionManager<PgConnection>>>, cached_user: CachedUser, post_data: Json<Post>) -> (Status, String) {
    match pool.get() {
        Ok(conn) => {
            create_user_post(conn, cached_user.user.user_id, post_data.title.clone(), post_data.description.clone() );
            (Status::Ok, json!({}).to_string())
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
