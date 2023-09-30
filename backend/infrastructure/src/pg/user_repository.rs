use sqlx::Postgres;

pub struct PgUserRepository {
    transaction: sqlx::Transaction<'static, Postgres>,
}

impl From<sqlx::Transaction<'static, Postgres>> for PgUserRepository {
    fn from(transaction: sqlx::Transaction<'static, Postgres>) -> Self {
        Self { transaction }
    }
}

impl app::api::Transaction for PgUserRepository {
    type Error = sqlx::Error;

    async fn commit(self) -> Result<(), Self::Error> {
        self.transaction.commit().await
    }

    async fn rollback(self) -> Result<(), Self::Error> {
        self.transaction.rollback().await
    }
}

impl app::api::UserRepository for PgUserRepository {
    async fn save(&mut self, user: domain::User<()>) -> Result<domain::User, Self::Error> {
        let new_row = sqlx::query!(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email, password;",
            &*user.email,
            &*user.password
        )
        .fetch_one(&mut *self.transaction)
        .await?;

        Ok(domain::User {
            id: new_row.id,
            email: new_row.email.into(),
            password: new_row.password.into(),
        })
    }

    async fn find_by_email(&mut self, email: &str) -> Result<Option<domain::User>, Self::Error> {
        let record = sqlx::query!("SELECT * FROM users WHERE users.email = $1", email)
            .fetch_one(&mut *self.transaction)
            .await;

        match record {
            Ok(value) => Ok(Some(domain::User {
                id: value.id,
                email: value.email.into(),
                password: value.password.into(),
            })),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }
}
