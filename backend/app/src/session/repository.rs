use std::num::NonZeroI32;

use async_trait::async_trait;

use super::Session;
use crate::ports::{EntityAlreadyExistError, EntityDoesNotExistError};

#[async_trait]
pub trait SessionRepository {
    async fn count_not_expired_by_user_id(&self, id: NonZeroI32) -> Result<usize, anyhow::Error>;

    async fn find(&self, id: NonZeroI32, metadata: &str) -> Result<Option<Session>, anyhow::Error>;

    async fn insert(
        &self,
        session: Session,
    ) -> Result<Result<Session, EntityAlreadyExistError>, anyhow::Error>;

    async fn update(
        &self,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error>;

    async fn delete_all(&self, user_id: NonZeroI32) -> Result<Vec<Session>, anyhow::Error>;

    async fn delete(
        &self,
        id: NonZeroI32,
        metadata: &str,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error>;
}
