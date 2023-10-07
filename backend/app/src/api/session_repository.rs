use async_trait::async_trait;
use domain::Session;

use super::{EntityAlreadyExistError, EntityDoesNotExistError};

#[async_trait]
pub trait SessionRepository {
    type Transaction;

    async fn count_not_expired_by_user_id(
        &self,
        t: &mut Self::Transaction,
        user_id: i32,
    ) -> Result<usize, anyhow::Error>;

    async fn find(
        &self,
        t: &mut Self::Transaction,
        user_id: i32,
        metadata: &str,
    ) -> Result<Option<Session>, anyhow::Error>;

    async fn insert(
        &self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Result<Session, EntityAlreadyExistError>, anyhow::Error>;

    async fn update(
        &self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error>;

    async fn delete(
        &self,
        t: &mut Self::Transaction,
        session: Session,
    ) -> Result<Result<Session, EntityDoesNotExistError>, anyhow::Error>;
}
