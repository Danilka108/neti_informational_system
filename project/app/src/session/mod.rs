mod params;
mod repository;
mod seconds;
mod service;

use std::num::NonZeroI32;

use anyhow::Context;

pub use params::{SessionTTL, SessionsMaxNumber};
pub use repository::SessionRepository;
pub use seconds::{Seconds, SecondsFromUnixEpoch};
pub use service::{
    DeleteSessionException, SaveSessionException, SessionService, UpdateSessionException,
    ValidateSessionException,
};

use crate::{user::User, Ref};

pub type DynSessionRepository = Box<dyn SessionRepository + Send + Sync>;

#[derive(Debug, Hash, Clone)]
pub struct Session {
    pub user_id: Ref<NonZeroI32, User>,
    pub expires_at: SecondsFromUnixEpoch,
    pub metadata: String,
    pub refresh_token: String,
}

impl Session {
    pub fn new(
        id: NonZeroI32,
        metadata: String,
        refresh_token: String,
        ttl: Seconds,
    ) -> Result<Self, anyhow::Error> {
        let expires_at =
            SecondsFromUnixEpoch::from_ttl(ttl).context("failed to generate a new 'expires at'")?;

        Ok(Self {
            user_id: Ref::from(id),
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
        self.expires_at =
            SecondsFromUnixEpoch::from_ttl(ttl).context("failed to generate a new 'expires at'")?;

        Ok(self)
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.user_id.eq(&other.user_id) && self.metadata.eq(&other.metadata)
    }
}

impl Eq for Session {}
