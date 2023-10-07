use anyhow::Context;
use app::api::{EntityAlreadyExistError, EntityDoesNotExistError, UserRepository};
use async_trait::async_trait;
use domain::User;

use super::PgTransaction;

pub struct PgUserRepository;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "UPPERCASE")]
pub(super) enum PgRole {
    Admin,
}

impl From<domain::Role> for PgRole {
    fn from(value: domain::Role) -> PgRole {
        match value {
            domain::Role::Admin => Self::Admin,
        }
    }
}

impl From<PgRole> for domain::Role {
    fn from(value: PgRole) -> Self {
        match value {
            PgRole::Admin => Self::Admin,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct PgUser {
    id: i32,
    email: String,
    password: Vec<u8>,
    role: PgRole,
}

impl From<PgUser> for domain::User {
    fn from(
        PgUser {
            id,
            email,
            password,
            role,
        }: PgUser,
    ) -> Self {
        Self {
            id,
            email,
            password,
            role: role.into(),
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    type Transaction = PgTransaction;

    async fn insert(
        &self,
        t: &mut Self::Transaction,
        user: domain::User<()>,
    ) -> Result<Result<User, EntityAlreadyExistError>, anyhow::Error> {
        let insert_result = sqlx::query_as!(
            PgUser,
            r#"
                INSERT
                    INTO users (email, password, role)
                    VALUES ($1, $2, $3)
                    ON CONFLICT DO NOTHING
                    RETURNING id, email, password, role as "role!: PgRole";
            "#,
            &*user.email,
            &*user.password,
            PgRole::from(user.role) as PgRole,
        )
        .fetch_optional(&mut **t)
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
        t: &mut Self::Transaction,
        user: domain::User,
    ) -> Result<Result<User, EntityDoesNotExistError>, anyhow::Error> {
        let update_result = sqlx::query_as!(
            PgUser,
            r#"
                UPDATE users
                    SET
                        email = $1,
                        password = $2,
                        role = $3
                    WHERE id = $4
                    RETURNING id, email, password, role as "role!: PgRole";
            "#,
            &*user.email,
            &*user.password,
            PgRole::from(user.role) as PgRole,
            user.id,
        )
        .fetch_optional(&mut **t)
        .await;

        match update_result {
            Ok(Some(val)) => Ok(Ok(val.into())),
            Ok(None) => Ok(Err(EntityDoesNotExistError)),
            Err(err) => Err(err),
        }
        .context("failed to insert new row into users table")
    }

    async fn find_by_email(
        &self,
        t: &mut Self::Transaction,
        email: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let record = sqlx::query_as!(
            PgUser,
            r#"
            SELECT id, email, password, role as "role!: PgRole"
                FROM users
                WHERE users.email = $1
            "#,
            email
        )
        .fetch_optional(&mut **t)
        .await;

        Ok(record
            .context("failed to select row from users table")?
            .map(Into::into))
    }

    async fn find_by_id(
        &self,
        t: &mut Self::Transaction,
        id: i32,
    ) -> Result<Option<User>, anyhow::Error> {
        let record = sqlx::query_as!(
            PgUser,
            r#"
            SELECT id, email, password, role as "role!: PgRole"
                FROM users
                WHERE users.id = $1
            "#,
            id
        )
        .fetch_optional(&mut **t)
        .await;

        Ok(record
            .context("failed to select row from users table")?
            .map(Into::into))
    }
}
