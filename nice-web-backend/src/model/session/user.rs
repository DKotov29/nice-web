use std::sync::Arc;
use rocket::request::FromRequest;
use crate::model::db::{ User};
use rocket::http::Status;
use rocket::request::{Outcome};
use rocket::{Request, State};

use crate::model::session::SessionManager;
pub struct CachedUser {
    pub user: User,
    pub session: u128
}

#[derive(Debug)]
pub enum TempEnumForUser {
    SessionExpired,
}

#[async_trait]
impl<'r> FromRequest<'r> for CachedUser {
    type Error = TempEnumForUser;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // it's better to put a session manager in Arc (?) todo
        let sessions = request
            .guard::<&State<Arc<SessionManager>>>()
            .await
            .expect("unexpected behavior");

        if let Some(session) = request.headers().get_one("session-token") {
            // there's a one problem: every route will get cached user info, posiibly its not so bad
            let session_id = session.parse::<u128>();

            if session_id.is_ok() {
                let session_id = session_id.unwrap();
                if let Some(user) = sessions.get_session(session_id)
                {
                    return Outcome::Success(CachedUser {
                        user,
                        session: session_id,
                    });
                }
            }
        }
        Outcome::Failure((Status::Unauthorized, TempEnumForUser::SessionExpired))
    }
}
