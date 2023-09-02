use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{
    json,
    Value
};

use super::super::super::model::db::is_user_password_good_and_user_exists;
use crate::controller::routes::Credentials;
use crate::model::db::CheckUserResult;
use crate::model::session::SessionManager;

#[post("/signin", format = "json", data = "<credentials>")]
pub fn signin(
    pool: &State<Pool<ConnectionManager<PgConnection>>>,
    session_manager: &State<SessionManager>,
    credentials: Json<Credentials>,
) -> (Status, Json<Value>) {
    match pool.get() {
        Ok(conn) => {
            match is_user_password_good_and_user_exists(
                conn,
                credentials.username.as_str(),
                credentials.password.as_str(),
            ) {
                CheckUserResult::Ok(user) => {
                    let session_id = session_manager.add_session(user);
                    (
                        Status::Ok,
                        Json(json!({
                             "token": session_id.to_string()
                        })),
                    )
                }
                CheckUserResult::NoSuchUser | CheckUserResult::NoSuchPassword => (
                    Status::BadRequest,
                    Json(json!({
                         "error": "Password or/and username incorrect or user dont exist"
                    })),
                ),
            }
        }
        Err(err) => {
            println!("Pool dont work, check it, error message: {}", err.to_string()); // todo some logging?
            (
                Status::BadRequest,
                Json(json!({
                    "error": "Unfortunately, server cannot response due to database problem"
                })),
            )
        }
    }
}
