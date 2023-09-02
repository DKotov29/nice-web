pub mod user;

use std::time::Duration;

use moka::sync::{Cache, CacheBuilder};
use rand::RngCore;

use crate::model::db::User;
const CACHE_TTL: u64 = 60 * 60;

pub struct SessionManager {
    users: Cache<u128, User>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            users: CacheBuilder::default()
                .time_to_live(Duration::from_secs(CACHE_TTL))
                .build(),
        }
    }

    pub fn get_session(&self, session_id: u128) -> Option<User> {
        self.users.get(&session_id)
    }

    pub fn add_session(&self, user: User) -> u128 {
        let session_token = self.generate_unique_session_id();
        self.users.insert(session_token, user);
        session_token
    }

    fn generate_unique_session_id(&self) -> u128 {
        let mut session_token = 0u128;
        while session_token == 0 || self.users.get(&session_token).is_some() {
            let mut u128_pool = [0u8; 16];
            rand::thread_rng().fill_bytes(&mut u128_pool);
            session_token = u128::from_le_bytes(u128_pool);
        }
        session_token
    }

    pub fn invalidate_session(&self, session_id: u128) {
        self.users.invalidate(&session_id)
    }
}
