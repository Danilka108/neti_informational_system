mod models;

use std::sync::Arc;

use app::ports::{
    EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError, UserRepository,
};
use app::user::User;
use app::{Outcome, SerialId};
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

use models::{PgRole, PgUser};

use super::ProvideTxn;

pub struct PgUserRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

impl ProvideTxn for PgUserRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn insert(&self, user: User) -> Outcome<User, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgUser,
            r#"
                INSERT
                    INTO users (id, email, hashed_password, role)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT DO NOTHING
                    RETURNING id, email, hashed_password, role as "role!: PgRole";
            "#,
            *user.id,
            &user.email,
            &user.hashed_password.0,
            PgRole::from(user.role) as PgRole,
        ))
        .await
    }

    async fn update(&self, user: User) -> Outcome<User, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgUser,
            r#"
                UPDATE users
                    SET
                        email = $1,
                        hashed_password = $2,
                        role = $3
                    WHERE id = $4
                    RETURNING id, email, hashed_password, role as "role!: PgRole";
            "#,
            &user.email,
            &user.hashed_password.0,
            PgRole::from(user.role) as PgRole,
            *user.id,
        ))
        .await
    }

    async fn find_by_email(&self, email: &str) -> Outcome<User, EntityNotFoundError> {
        self.fetch_optional(sqlx::query_as!(
            PgUser,
            r#"
                SELECT id, email, hashed_password, role as "role!: PgRole"
                    FROM users
                    WHERE users.email = $1
            "#,
            email,
        ))
        .await
    }

    async fn find_by_id(&self, id: SerialId) -> Outcome<User, EntityNotFoundError> {
        self.fetch_optional(sqlx::query_as!(
            PgUser,
            r#"
                SELECT id, email, hashed_password, role as "role!: PgRole"
                    FROM users
                    WHERE users.id = $1;
            "#,
            id,
        ))
        .await
    }
}
