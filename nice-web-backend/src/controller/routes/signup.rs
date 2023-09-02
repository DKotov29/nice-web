use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;

use crate::controller::routes::Credentials;
use crate::model::db::{create_user_if_unique, CreateUserResult};

#[post("/signup", data = "<credentials>")]
pub fn signup(pool: &State<Pool<ConnectionManager<PgConnection>>>, credentials: Json<Credentials>) -> (Status, String) {
    if credentials.password.chars().any(|char| !char.is_alphanumeric()) {
        return (
            Status::BadRequest,
            json!({"error" : "This password cannot be used. Use only alphabetic characters or numbers"}).to_string(),
        );
    }
    match pool.get() {
        Ok(conn) => match create_user_if_unique(conn, credentials.username.as_str(), credentials.password.as_str()) {
            CreateUserResult::Ok => {
                (Status::Ok, json!({}).to_string())
            }
            CreateUserResult::SuchUserExists => {
                (Status::BadRequest, json!({"error" : "This username is occupied, try another"}).to_string())
            }
            CreateUserResult::PasswordNotValidForHash => {
                (Status::BadRequest, json!({"error" : "This password cannot be used. (Not valid for using in database)"}).to_string())
            }
        },
        Err(err) => {
            eprintln!("Pool dont work, check it, error message: {}", err.to_string());
            (Status::BadRequest, json!({"error" : "Unfortunately, server cannot response due to database problem"}).to_string()) // server side problem?
        }
    }
}
