use async_trait::async_trait;

use super::Session;
use crate::ports::{EntityAlreadyExistError, EntityDoesNotExistError};

#[async_trait]
pub trait SessionRepository {
    async fn count_not_expired_by_user_id(&self, user_id: i32) -> Result<usize, anyhow::Error>;

    async fn find(&self, user_id: i32, metadata: &str) -> Result<Option<Session>, anyhow::Error>;

    async fn insert(
        &self,
        session: Session,
    ) -> Result<Result<Session, EntityAlreadyExistError>, anyhow::Error>;

    async fn update(
        &self,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error>;

    async fn delete_all(&self, user_id: i32) -> Result<Vec<Session>, anyhow::Error>;

    async fn delete(
        &self,
        user_id: i32,
        metadata: &str,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error>;
}
