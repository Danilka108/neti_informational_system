use anyhow::Context;
use app::api::{EntityAlreadyExistError, EntityDoesNotExistError, SessionRepository};
use async_trait::async_trait;
use domain::Session;

use super::PgTransaction;

pub struct PgSessionRepository;

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
            user_id,
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
            user_id,
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
    type Transaction = PgTransaction;

    async fn count_not_expired_by_user_id(
        &self,
        t: &mut Self::Transaction,
        user_id: i32,
    ) -> Result<usize, anyhow::Error> {
        let result = sqlx::query!(
            r#"
                SELECT
                    COUNT(*) as "count!"
                    FROM sessions
                    WHERE user_id = $1;
            "#,
            user_id
        )
        .fetch_one(&mut **t)
        .await
        .context("failed to count rows in users table")?;

        let result = usize::try_from(result.count).context("failed to convert i64 to usize")?;

        Ok(result)
    }

    async fn find(
        &self,
        t: &mut Self::Transaction,
        user_id: i32,
        metadata: &str,
    ) -> Result<Option<Session>, anyhow::Error> {
        let result = sqlx::query_as!(
            PgSession,
            r#"
                SELECT
                    user_id, metadata, refresh_token, expires_at_in_seconds
                    FROM sessions
                    WHERE user_id = $1 and metadata = $2;
            "#,
            user_id,
            metadata,
        )
        .fetch_optional(&mut **t)
        .await
        .context("failed to select row from sessions table")?;

        result.map(TryInto::try_into).transpose()
    }

    async fn insert(
        &self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Result<Session, EntityAlreadyExistError>, anyhow::Error> {
        let expires_at = i64::try_from(session.expires_at.seconds.val)
            .context("failed to convert u64 to i64")?;

        let insert_result = sqlx::query_as!(
            PgSession,
            r#"
                INSERT
                    INTO sessions (user_id, metadata, refresh_token, expires_at_in_seconds)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT DO NOTHING
                    RETURNING *;
            "#,
            session.user_id,
            &session.metadata,
            &session.refresh_token,
            expires_at,
        )
        .fetch_optional(&mut **t)
        .await
        .context("failed to insert new row into users table")?;

        insert_result
            .map(TryInto::try_into)
            .transpose()
            .map(|v| v.ok_or(EntityAlreadyExistError))
    }

    async fn update(
        &self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error> {
        let expires_at = i64::try_from(session.expires_at.seconds.val)
            .context("failed to convert u64 to i64")?;

        let insert_result = sqlx::query_as!(
            PgSession,
            r#"
                UPDATE
                    sessions
                    SET
                        refresh_token = $3,
                        expires_at_in_seconds = $4
                    WHERE
                        user_id = $1 AND metadata = $2
                    RETURNING *;
            "#,
            session.user_id,
            &session.metadata,
            &session.refresh_token,
            expires_at,
        )
        .fetch_optional(&mut **t)
        .await
        .context("failed to insert new row into users table")?;

        insert_result
            .map(TryInto::try_into)
            .transpose()
            .map(|v| v.ok_or(EntityDoesNotExistError))
    }

    async fn delete(
        &self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error> {
        let delete_result = sqlx::query_as!(
            PgSession,
            r#"
                DELETE
                    FROM sessions
                    WHERE user_id = $1 AND metadata = $2
                    RETURNING *;
            "#,
            session.user_id,
            &session.metadata
        )
        .fetch_optional(&mut **t)
        .await
        .context("failed to delete row from sessions table")?;

        delete_result
            .map(TryInto::try_into)
            .transpose()
            .map(|v| v.ok_or(EntityDoesNotExistError))
    }
}
