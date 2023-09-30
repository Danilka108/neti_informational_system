use crate::api::{PasswordEncoder, Transaction, TransactionBuilder, UserRepository};
use domain::User;

pub struct UserService<T, P> {
    tx_builder: T,
    password_encoder: P,
}

enum UserServiceError {
    EmailAlreadyInUse,
    InvalidEmailOrPassword,
    RepoError(Box<dyn std::error::Error>),
}

impl<T, P> UserService<T, P>
where
    T: TransactionBuilder,
    T::Transaction: UserRepository,
    P: PasswordEncoder,
{
    pub fn new(transaction_builder: T, password_encoder: P) -> Self {
        Self {
            tx_builder: transaction_builder,
            password_encoder,
        }
    }

    pub async fn create(
        self,
        email: Box<str>,
        password: Box<str>,
    ) -> Result<User, UserServiceError> {
        let mut tx = self.tx_builder.begin().await?;

        let None = tx.find_by_email(&email).await? else {
            return Err(UserServiceError::EmailAlreadyInUse);
        };

        let encoded_password = self.password_encoder.encode(&password);

        let user = tx
            .save(User {
                id: (),
                email,
                password: encoded_password,
            })
            .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn authorize(self, email: &str, password: &str) -> Result<User, UserServiceError> {
        let mut tx = self.tx_builder.begin().await?;

        let Some(user) = tx.find_by_email(&email).await? else {
            return Err(UserServiceError::InvalidEmailOrPassword);
        };

        let is_password_matches = self.password_encoder.is_matches(password, &user.password);

        if !is_password_matches {
            return Err(UserServiceError::InvalidEmailOrPassword);
        }

        tx.commit().await?;
        Ok(user)
    }
}

impl<E: std::error::Error + 'static> From<E> for UserServiceError {
    fn from(value: E) -> Self {
        Self::RepoError(Box::new(value))
    }
}
