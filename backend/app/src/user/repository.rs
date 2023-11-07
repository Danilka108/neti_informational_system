use async_trait::async_trait;

use super::User;
use crate::ports::{EntityAlreadyExistError, EntityDoesNotExistError};

#[async_trait]
pub trait UserRepository {
    async fn insert(
        &self,
        user: User,
    ) -> Result<Result<User, EntityAlreadyExistError>, anyhow::Error>;

    async fn update(
        &self,
        user: User,
    ) -> Result<Result<User, EntityDoesNotExistError>, anyhow::Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, anyhow::Error>;
}
