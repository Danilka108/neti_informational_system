use std::convert::Infallible;

use async_trait::async_trait;

use super::Session;
use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome, SerialId,
};

#[async_trait]
pub trait SessionRepository {
    async fn count_not_expired_by_user_id(&self, id: SerialId) -> Outcome<usize, Infallible>;

    async fn find(&self, id: SerialId, metadata: &str) -> Outcome<Session, EntityNotFoundError>;

    async fn insert(&self, session: Session) -> Outcome<Session, EntityAlreadyExistError>;

    async fn update(&self, session: Session) -> Outcome<Session, EntityDoesNotExistError>;

    async fn delete_all(&self, user_id: SerialId) -> Outcome<Vec<Session>, Infallible>;

    async fn delete(
        &self,
        id: SerialId,
        metadata: &str,
    ) -> Outcome<Session, EntityDoesNotExistError>;
}
