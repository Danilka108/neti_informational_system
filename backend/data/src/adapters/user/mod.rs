mod pg_user;

use std::sync::Arc;

use anyhow::Context;
use app::ports::{EntityAlreadyExistError, EntityDoesNotExistError, UserRepository};
use app::user::User;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

use pg_user::{PgRole, PgUser};

pub struct PgUserRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn insert(
        &self,
        user: User,
    ) -> Result<Result<User, EntityAlreadyExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let insert_result = sqlx::query_file_as!(
            PgUser,
            "src/adapters/user/scripts/insert.sql",
            *user.id,
            &*user.email,
            &*user.hashed_password.0,
            PgRole::from(user.role) as PgRole,
        )
        .fetch_optional(conn)
        .await;

        match insert_result {
            Ok(Some(val)) => Ok(Ok(val.into())),
            Ok(None) => Ok(Err(EntityAlreadyExistError)),
            Err(err) => Err(err),
        }
        .context("failed to insert new row into users table")
    }

    async fn update(
        &self,
        user: User,
    ) -> Result<Result<User, EntityDoesNotExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let update_result = sqlx::query_file_as!(
            PgUser,
            "src/adapters/user/scripts/update.sql",
            &*user.email,
            &*user.hashed_password.0,
            PgRole::from(user.role) as PgRole,
            *user.id,
        )
        .fetch_optional(conn)
        .await;

        match update_result {
            Ok(Some(val)) => Ok(Ok(val.into())),
            Ok(None) => Ok(Err(EntityDoesNotExistError)),
            Err(err) => Err(err),
        }
        .context("failed to update new row in users table")
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let record =
            sqlx::query_file_as!(PgUser, "src/adapters/user/scripts/find_by_email.sql", email)
                .fetch_optional(conn)
                .await;

        Ok(record
            .context("failed to select row from users table")?
            .map(Into::into))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let record = sqlx::query_file_as!(PgUser, "src/adapters/user/scripts/find_by_id.sql", id)
            .fetch_optional(conn)
            .await;

        Ok(record
            .context("failed to select row from users table")?
            .map(Into::into))
    }
}
