mod params;
mod repository;
mod seconds;
mod service;

use anyhow::Context;
use serde::{Deserialize, Serialize};

pub use params::{SessionTTL, SessionsMaxNumber};
pub use repository::SessionRepository;
pub use seconds::{Seconds, SecondsFromUnixEpoch};
pub use service::{
    DeleteSessionError, SaveSessionError, SessionService, UpdateSessionError, ValidateSessionError,
};

pub type DynSessionRepository = Box<dyn SessionRepository + Send + Sync>;

#[derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionOwned {
    pub metadata: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Session {
    pub user_id: i32,
    pub expires_at: SecondsFromUnixEpoch,
    pub metadata: String,
    pub refresh_token: String,
}

impl Session {
    pub fn new(
        user_id: i32,
        metadata: String,
        refresh_token: String,
        ttl: Seconds,
    ) -> Result<Self, anyhow::Error> {
        let expires_at = SecondsFromUnixEpoch::new_expires_at(ttl)
            .context("failed to generate a new 'expires at'")?;

        Ok(Self {
            user_id,
            expires_at,
            metadata,
            refresh_token,
        })
    }

    pub fn is_expired(&self) -> Result<bool, anyhow::Error> {
        self.expires_at.is_expired()
    }

    pub fn update(mut self, refresh_token: String, ttl: Seconds) -> Result<Self, anyhow::Error> {
        self.refresh_token = refresh_token;
        self.expires_at = SecondsFromUnixEpoch::new_expires_at(ttl)
            .context("failed to generate a new 'expires at'")?;

        Ok(self)
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.user_id.eq(&other.user_id) && self.metadata.eq(&other.metadata)
    }
}

impl Eq for Session {}
