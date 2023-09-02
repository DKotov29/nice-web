use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};

use crate::model::db::User;
use crate::model::session::SessionManager;
pub struct CachedUser {
    pub user: User,
    pub session: u128,
}

#[derive(Debug)]
pub enum TempEnumForUser {
    SessionExpired,
}

#[async_trait]
impl<'r> FromRequest<'r> for CachedUser {
    type Error = TempEnumForUser;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let sessions = request
            .guard::<&State<SessionManager>>()
            .await
            .expect("unexpected behavior of getting sessionmanager");

        if let Some(session) = request.headers().get_one("session-token") {
            let session_id = session.parse::<u128>();
            if let Ok(session_id) = session_id {
                if let Some(user) = sessions.get_session(session_id) {
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
