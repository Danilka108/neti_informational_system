use std::{convert::Infallible, num::NonZeroI32};

use async_trait::async_trait;

use super::Session;
use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome,
};

#[async_trait]
pub trait SessionRepository {
    async fn count_not_expired_by_user_id(&self, id: NonZeroI32) -> Outcome<usize, Infallible>;

    async fn find(&self, id: NonZeroI32, metadata: &str) -> Outcome<Session, EntityNotFoundError>;

    async fn insert(&self, session: Session) -> Outcome<Session, EntityAlreadyExistError>;

    async fn update(&self, session: Session) -> Outcome<Session, EntityDoesNotExistError>;

    async fn delete_all(&self, user_id: NonZeroI32) -> Outcome<Vec<Session>, Infallible>;

    async fn delete(
        &self,
        id: NonZeroI32,
        metadata: &str,
    ) -> Outcome<Session, EntityDoesNotExistError>;
}
