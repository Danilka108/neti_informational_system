use anyhow::Context;
use app::session::Session;
use app::Ref;

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
            user_id: *user_id,
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

impl TryFrom<PgSession> for Session {
    type Error = anyhow::Error;

    fn try_from(
        PgSession {
            user_id,
            metadata,
            refresh_token,
            expires_at_in_seconds,
        }: PgSession,
    ) -> Result<Self, Self::Error> {
        Ok(Session {
            user_id: Ref::from(user_id),
            metadata,
            refresh_token,
            expires_at: u64::try_from(expires_at_in_seconds)
                .context("failed to convert i64 to seconds")?
                .into(),
        })
    }
}
