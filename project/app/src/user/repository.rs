use async_trait::async_trait;

use super::User;
use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome, SerialId,
};

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Outcome<User, EntityAlreadyExistError>;

    async fn update(&self, user: User) -> Outcome<User, EntityDoesNotExistError>;

    async fn find_by_email(&self, email: &str) -> Outcome<User, EntityNotFoundError>;

    async fn find_by_id(&self, id: SerialId) -> Outcome<User, EntityNotFoundError>;
}
