use std::num::NonZeroI32;

use anyhow::Context;

use crate::{
    person::{CreatePersonException, Person, PersonService},
    Outcome, Ref,
};

use super::{DynPasswordHasher, DynUserRepository, Role, User};

pub struct UserService {
    pub(crate) person_service: PersonService,
    pub(crate) repo: DynUserRepository,
    pub(crate) pass_hasher: DynPasswordHasher,
}

impl UserService {
    pub(crate) async fn find_by_id(self, id: NonZeroI32) -> Result<Option<User>, anyhow::Error> {
        self.repo.find_by_id(id).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticateUserException {
    #[error("invalid login or password")]
    InvalidLoginOrPassword,
}

impl UserService {
    pub(crate) async fn authenticate(
        self,
        email: &str,
        password: &str,
    ) -> Outcome<User, AuthenticateUserException> {
        let maybe_user = self.repo.find_by_email(email).await?;

        let Some(user) = maybe_user else {
            return Outcome::Exception(AuthenticateUserException::InvalidLoginOrPassword);
        };

        let is_password_not_matches = !self
            .pass_hasher
            .is_matches(password, &user.hashed_password)
            .await?;

        if is_password_not_matches {
            return Outcome::Exception(AuthenticateUserException::InvalidLoginOrPassword);
        }

        Outcome::Success(user)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateUserException {
    #[error(transparent)]
    FailedToCreatePerson(#[from] CreatePersonException),
    #[error("email already in use")]
    EmailAlreadyInUse,
}

impl UserService {
    pub async fn create(
        self,
        email: String,
        password: String,
        role: Role,
    ) -> Outcome<User, CreateUserException> {
        let is_email_already_in_use = self.repo.find_by_email(&email).await?.is_some();

        if is_email_already_in_use {
            return Outcome::Exception(CreateUserException::EmailAlreadyInUse);
        }

        let hashed_password = self.pass_hasher.hash(password).await?;

        let Person { id: person_id, .. } = self.person_service.create().await?;

        let user = User {
            id: Ref::from(NonZeroI32::try_from(person_id).unwrap()),
            role,
            email,
            hashed_password,
        };

        let user = self
            .repo
            .insert(user)
            .await?
            .context("user not existence is checked before inserting, but an error occurs")?;

        Outcome::Success(user)
    }
}
