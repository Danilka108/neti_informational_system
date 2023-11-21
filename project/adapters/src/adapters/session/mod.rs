mod models;

use anyhow::Context;
use app::Outcome;
use async_trait::async_trait;
use std::convert::Infallible;
use std::num::NonZeroI32;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;
use app::ports::{
    EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError, SessionRepository,
};
use app::session::Session;

use models::{CountResult, PgSession};

use super::ProvideTxn;

pub struct PgSessionRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

impl ProvideTxn for PgSessionRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn count_not_expired_by_user_id(
        &self,
        user_id: NonZeroI32,
    ) -> Outcome<usize, Infallible> {
        self.fetch_one(sqlx::query_as!(
            CountResult,
            r#"SELECT COUNT(*) as "count!" FROM user_sessions WHERE user_id = $1;"#,
            user_id.get()
        ))
        .await
    }

    async fn find(
        &self,
        user_id: NonZeroI32,
        metadata: &str,
    ) -> Outcome<Session, EntityNotFoundError> {
        self.fetch_optional(sqlx::query_as!(
            PgSession,
            "
                SELECT user_id, metadata, refresh_token, expires_at_in_seconds
                    FROM user_sessions
                    WHERE user_id = $1 and metadata = $2;
            ",
            user_id.get(),
            &metadata
        ))
        .await
    }

    async fn insert(&self, session: Session) -> Outcome<Session, EntityAlreadyExistError> {
        let expires_at = i64::try_from(session.expires_at.seconds.val)
            .context("failed to convert u64 to i64")? as i32;

        self.fetch_optional(sqlx::query_as!(
            PgSession,
            "
                INSERT
                    INTO user_sessions (user_id, metadata, refresh_token, expires_at_in_seconds)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT DO NOTHING
                    RETURNING *;
            ",
            session.user_id.get(),
            &session.metadata,
            &session.refresh_token,
            expires_at,
        ))
        .await
    }

    async fn update(&self, session: Session) -> Outcome<Session, EntityDoesNotExistError> {
        let expires_at = i64::try_from(session.expires_at.seconds.val)
            .context("failed to convert u64 to i64")? as i32;

        self.fetch_optional(sqlx::query_as!(
            PgSession,
            "
                UPDATE
                    user_sessions
                    SET
                        refresh_token = $3,
                        expires_at_in_seconds = $4
                    WHERE
                        user_id = $1 AND metadata = $2
                    RETURNING *;
            ",
            session.user_id.get(),
            &session.metadata,
            &session.refresh_token,
            expires_at,
        ))
        .await
    }

    async fn delete_all(&self, user_id: NonZeroI32) -> Outcome<Vec<Session>, Infallible> {
        self.fetch_all(sqlx::query_as!(
            PgSession,
            "
                DELETE
                    FROM user_sessions
                    WHERE user_id = $1
                    RETURNING *;
            ",
            user_id.get(),
        ))
        .await
    }

    async fn delete(
        &self,
        user_id: NonZeroI32,
        metadata: &str,
    ) -> Outcome<Session, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgSession,
            "
                DELETE
                    FROM user_sessions
                    WHERE user_id = $1 AND metadata = $2
                    RETURNING *;
            ",
            user_id.get(),
            metadata,
        ))
        .await
    }
}
