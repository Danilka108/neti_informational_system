use std::sync::Arc;

use anyhow::Context;
use app::ports::{EntityAlreadyExistError, EntityDoesNotExistError, SessionRepository};
use app::session::Session;
use app::Ref;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

pub struct PgSessionRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

struct PgSession {
    user_id: i32,
    metadata: String,
    refresh_token: String,
    expires_at_in_seconds: i64,
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

#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn count_not_expired_by_user_id(&self, user_id: i32) -> Result<usize, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let result = sqlx::query!(
            r#"
                SELECT
                    COUNT(*) as "count!"
                    FROM user_sessions
                    WHERE user_id = $1;
            "#,
            user_id
        )
        .fetch_one(conn)
        .await
        .context("failed to count rows in users table")?;

        let result = usize::try_from(result.count).context("failed to convert i64 to usize")?;

        Ok(result)
    }

    async fn find(&self, user_id: i32, metadata: &str) -> Result<Option<Session>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let result = sqlx::query_as!(
            PgSession,
            r#"
                SELECT
                    user_id, metadata, refresh_token, expires_at_in_seconds
                    FROM user_sessions
                    WHERE user_id = $1 and metadata = $2;
            "#,
            user_id,
            metadata,
        )
        .fetch_optional(conn)
        .await
        .context("failed to select row from sessions table")?;

        result.map(TryInto::try_into).transpose()
    }

    async fn insert(
        &self,
        session: Session,
    ) -> Result<Result<Session, EntityAlreadyExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let expires_at = i64::try_from(session.expires_at.seconds.val)
            .context("failed to convert u64 to i64")? as i32;

        let insert_result = sqlx::query_as!(
            PgSession,
            r#"
                INSERT
                    INTO user_sessions (user_id, metadata, refresh_token, expires_at_in_seconds)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT DO NOTHING
                    RETURNING *;
            "#,
            *session.user_id,
            &session.metadata,
            &session.refresh_token,
            expires_at,
        )
        .fetch_optional(conn)
        .await
        .context("failed to insert new row into sessions table")?;

        insert_result
            .map(TryInto::try_into)
            .transpose()
            .map(|v| v.ok_or(EntityAlreadyExistError))
    }

    async fn update(
        &self,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let expires_at = i64::try_from(session.expires_at.seconds.val)
            .context("failed to convert u64 to i64")? as i32;

        let insert_result = sqlx::query_as!(
            PgSession,
            r#"
                UPDATE
                    user_sessions
                    SET
                        refresh_token = $3,
                        expires_at_in_seconds = $4
                    WHERE
                        user_id = $1 AND metadata = $2
                    RETURNING *;
            "#,
            *session.user_id,
            &session.metadata,
            &session.refresh_token,
            expires_at,
        )
        .fetch_optional(conn)
        .await
        .context("failed to update row in sessions table")?;

        insert_result
            .map(TryInto::try_into)
            .transpose()
            .map(|v| v.ok_or(EntityDoesNotExistError))
    }

    async fn delete_all(&self, user_id: i32) -> Result<Vec<Session>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        sqlx::query_as!(
            PgSession,
            r#"
                DELETE
                    FROM user_sessions
                    WHERE user_id = $1
                    RETURNING *;
            "#,
            user_id,
        )
        .fetch_all(conn)
        .await
        .context("failed to delete all rows in sessions table")?
        .into_iter()
        .map(TryInto::try_into)
        .try_collect()
    }

    async fn delete(
        &self,
        user_id: i32,
        metadata: &str,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let delete_result = sqlx::query_as!(
            PgSession,
            r#"
                DELETE
                    FROM user_sessions
                    WHERE user_id = $1 AND metadata = $2
                    RETURNING *;
            "#,
            user_id,
            metadata,
        )
        .fetch_optional(conn)
        .await
        .context("failed to delete row in sessions table")?;

        delete_result
            .map(TryInto::try_into)
            .transpose()
            .map(|v| v.ok_or(EntityDoesNotExistError))
    }
}
