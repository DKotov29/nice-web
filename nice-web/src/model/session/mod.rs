use std::time::Duration;
use moka::sync::{Cache, CacheBuilder};
use crate::model::db::User;

const CACHE_TTL: u64 = 60 * 60;

pub struct SessionManager {
    users: Cache<String, User>
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            users: CacheBuilder::default()
                .time_to_live(Duration::from_secs(CACHE_TTL))
                .build()
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<User> {
        self.users.get(session_id)
    }

    pub fn add_session(&self, session_id: &str, user: User) {
        self.users.insert(session_id.to_owned(), user)
    }

    pub fn invalidate_session(&self, session_id: &str) {
        self.users.invalidate(session_id)
    }
}