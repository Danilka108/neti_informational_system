use crate::api::{PasswordEncoder, UserRepository};
use anyhow::Context;
use domain::{Role, User};

pub struct UserService<Repo, Encoder> {
    repository: Repo,
    password_encoder: Encoder,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateError {
    #[error("email already in use")]
    EmailAlreadyInUse,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticateError {
    #[error("invalid login or password")]
    InvalidLoginOrPassword,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl<RepoTransaction, Repo, Encoder> UserService<Repo, Encoder>
where
    Repo: UserRepository<Transaction = RepoTransaction>,
    Encoder: PasswordEncoder,
{
    pub fn new(repository: Repo, password_encoder: Encoder) -> Self {
        Self {
            repository,
            password_encoder,
        }
    }

    pub(crate) async fn get_user(
        &self,
        transaction: &mut RepoTransaction,
        id: i32,
    ) -> Result<Option<User>, anyhow::Error> {
        self.repository
            .find(transaction, id)
            .await
            .context("failed to read from user repository")
    }

    pub async fn create(
        &mut self,
        transaction: &mut RepoTransaction,
        login: String,
        role: Role,
        password: String,
    ) -> Result<User, CreateError> {
        let is_email_already_in_use = self
            .repository
            .find_by_email(transaction, &login)
            .await
            .context("failed to read from repository")?
            .is_some();

        if is_email_already_in_use {
            return Err(CreateError::EmailAlreadyInUse);
        }

        let password = self.password_encoder.encode(&password);
        let user = User {
            id: (),
            email: login,
            role,
            password,
        };

        let user = self
            .repository
            .save(transaction, user)
            .await
            .context("failed to save to repository")?;

        Ok(user)
    }

    pub async fn authenticate(
        &mut self,
        transaction: &mut RepoTransaction,
        login: &str,
        password: &str,
    ) -> Result<User, AuthenticateError> {
        let maybe_user = self
            .repository
            .find_by_email(transaction, &login)
            .await
            .context("failed to read from repository")?;

        let Some(user) = maybe_user else {
            return Err(AuthenticateError::InvalidLoginOrPassword);
        };

        let is_password_matches = self.password_encoder.is_matches(password, &user.password);

        if !is_password_matches {
            return Err(AuthenticateError::InvalidLoginOrPassword);
        }

        Ok(user)
    }
}
