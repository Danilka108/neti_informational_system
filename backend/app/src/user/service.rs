use anyhow::Context;

use crate::person::{CreatePersonError, Person, PersonService};

use super::{DynPasswordHasher, DynUserRepository, Role, User};

pub struct UserService {
    pub(crate) person_service: PersonService,
    pub(crate) repo: DynUserRepository,
    pub(crate) pass_hasher: DynPasswordHasher,
}

impl UserService {
    pub(crate) async fn find_by_id(self, id: i32) -> Result<Option<User>, anyhow::Error> {
        self.repo.find_by_id(id).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticateUserError {
    #[error("invalid login or password")]
    InvalidLoginOrPassword,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl UserService {
    pub(crate) async fn authenticate(
        self,
        email: &str,
        password: &str,
    ) -> Result<User, AuthenticateUserError> {
        let maybe_user = self.repo.find_by_email(email).await?;

        let Some(user) = maybe_user else {
            return Err(AuthenticateUserError::InvalidLoginOrPassword);
        };

        let is_password_not_matches = !self
            .pass_hasher
            .is_matches(password, &user.hashed_password)
            .await?;

        if is_password_not_matches {
            return Err(AuthenticateUserError::InvalidLoginOrPassword);
        }

        Ok(user)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateUserError {
    #[error(transparent)]
    CreatePersonError(#[from] CreatePersonError),
    #[error("email already in use")]
    EmailAlreadyInUse,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl UserService {
    pub async fn create(
        self,
        email: String,
        password: String,
        role: Role,
    ) -> Result<User, CreateUserError> {
        let is_email_already_in_use = self.repo.find_by_email(&email).await?.is_some();

        if is_email_already_in_use {
            return Err(CreateUserError::EmailAlreadyInUse);
        }

        let hashed_password = self.pass_hasher.hash(password).await?;

        let Person { id: person_id, .. } = self.person_service.create().await?;

        let user = User {
            id: (),
            person_id,
            role,
            email,
            hashed_password,
        };

        let user = self
            .repo
            .insert(user)
            .await?
            .context("user not existence is checked before inserting, but an error occurs")?;

        Ok(user)
    }
}
