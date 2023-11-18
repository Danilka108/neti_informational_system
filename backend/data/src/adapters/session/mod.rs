mod pg_session;

use anyhow::Context;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;
use app::ports::{EntityAlreadyExistError, EntityDoesNotExistError, SessionRepository};
use app::session::Session;

use pg_session::PgSession;

pub struct PgSessionRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn count_not_expired_by_user_id(&self, user_id: i32) -> Result<usize, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let result = sqlx::query_file!(
            "src/adapters/session/scripts/count_not_expired_by_user_id.sql",
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

        let result = sqlx::query_file_as!(
            PgSession,
            "src/adapters/session/scripts/find.sql",
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

        let insert_result = sqlx::query_file_as!(
            PgSession,
            "src/adapters/session/scripts/insert.sql",
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

        let insert_result = sqlx::query_file_as!(
            PgSession,
            "src/adapters/session/scripts/update.sql",
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

        sqlx::query_file_as!(
            PgSession,
            "src/adapters/session/scripts/delete_all.sql",
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

        let delete_result = sqlx::query_file_as!(
            PgSession,
            "src/adapters/session/scripts/delete.sql",
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
