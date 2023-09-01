use rocket::http::Status;
use rocket::State;
use serde_json::json;
use crate::model::session::SessionManager;
use crate::model::session::user::CachedUser;

#[post("/signout")]
pub fn signout(user: CachedUser, session_manager: &State<SessionManager>) -> (Status, String) {
    session_manager.invalidate_session(user.session);
    (Status::Ok, json!({}).to_string())
}