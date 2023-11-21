use std::num::NonZeroI32;

use anyhow::Context;
use app::session::Session;
use app::Ref;

use crate::adapters::IntoEntity;

pub struct CountResult {
    pub count: i64,
}

impl IntoEntity<usize> for CountResult {
    fn into_entity(self) -> Result<usize, anyhow::Error> {
        self.count
            .try_into()
            .context("failed to convert i64 into usize")
    }
}

pub struct PgSession {
    pub user_id: i32,
    pub metadata: String,
    pub refresh_token: String,
    pub expires_at_in_seconds: i64,
}

impl TryFrom<Session> for PgSession {
    type Error = anyhow::Error;

    fn try_from(
        Session {
            user_id,
            metadata,
            refresh_token,
            expires_at,
        }: Session,
    ) -> Result<Self, Self::Error> {
        Ok(PgSession {
            user_id: (*user_id).get(),
            metadata,
            refresh_token,
            expires_at_in_seconds: expires_at
                .seconds
                .val
                .try_into()
                .context("failed to convert seconds to i64")?,
        })
    }
}

impl IntoEntity<Session> for PgSession {
    fn into_entity(self) -> Result<Session, anyhow::Error> {
        Ok(Session {
            user_id: Ref::from(NonZeroI32::try_from(self.user_id).unwrap()),
            metadata: self.metadata,
            refresh_token: self.refresh_token,
            expires_at: u64::try_from(self.expires_at_in_seconds)
                .context("failed to convert i64 to seconds")?
                .into(),
        })
    }
}
