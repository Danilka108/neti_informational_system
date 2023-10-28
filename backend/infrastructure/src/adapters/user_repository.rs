use std::sync::Arc;

use anyhow::Context;
use app::ports::{EntityAlreadyExistError, EntityDoesNotExistError, UserRepository};
use app::user::{HashedPassword, Role, User};
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

pub struct PgUserRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub(super) enum PgRole {
    Admin,
}

#[derive(Debug, sqlx::FromRow)]
struct PgUser {
    id: i32,
    person_id: i32,
    email: String,
    hashed_password: String,
    role: PgRole,
}

impl From<Role> for PgRole {
    fn from(value: Role) -> PgRole {
        match value {
            Role::Admin => Self::Admin,
        }
    }
}

impl From<PgRole> for Role {
    fn from(value: PgRole) -> Self {
        match value {
            PgRole::Admin => Self::Admin,
        }
    }
}

impl From<PgUser> for User {
    fn from(
        PgUser {
            id,
            person_id,
            email,
            hashed_password,
            role,
        }: PgUser,
    ) -> Self {
        Self {
            id,
            person_id,
            email,
            hashed_password: HashedPassword(hashed_password),
            role: role.into(),
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn insert(
        &self,
        user: User<()>,
    ) -> Result<Result<User, EntityAlreadyExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let insert_result = sqlx::query_as!(
            PgUser,
            r#"
                INSERT
                    INTO users (person_id, email, hashed_password, role)
                    VALUES ($4, $1, $2, $3)
                    ON CONFLICT DO NOTHING
                    RETURNING id, email, person_id, hashed_password, role as "role!: PgRole";
            "#,
            &*user.email,
            &*user.hashed_password.0,
            PgRole::from(user.role) as PgRole,
            user.person_id,
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

        let update_result = sqlx::query_as!(
            PgUser,
            r#"
                UPDATE users
                    SET
                        email = $1,
                        hashed_password = $2,
                        role = $3,
                        person_id = $4
                    WHERE id = $5
                    RETURNING id, person_id, email, hashed_password, role as "role!: PgRole";
            "#,
            &*user.email,
            &*user.hashed_password.0,
            PgRole::from(user.role) as PgRole,
            user.person_id,
            user.id,
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

        let record = sqlx::query_as!(
            PgUser,
            r#"
            SELECT id, person_id, email, hashed_password, role as "role!: PgRole"
                FROM users
                WHERE users.email = $1
            "#,
            email
        )
        .fetch_optional(conn)
        .await;

        Ok(record
            .context("failed to select row from users table")?
            .map(Into::into))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let record = sqlx::query_as!(
            PgUser,
            r#"
            SELECT id, person_id, email, hashed_password, role as "role!: PgRole"
                FROM users
                WHERE users.id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await;

        Ok(record
            .context("failed to select row from users table")?
            .map(Into::into))
    }
}
