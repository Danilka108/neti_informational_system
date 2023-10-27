use crate::ports::{EntityAlreadyExistError, EntityDoesNotExistError};

use super::Person;

#[async_trait::async_trait]
pub trait PersonRepository {
    async fn insert(
        &self,
        person: Person<()>,
    ) -> Result<Result<Person, EntityAlreadyExistError>, anyhow::Error>;

    async fn update(
        &self,
        person: Person,
    ) -> Result<Result<Person, EntityDoesNotExistError>, anyhow::Error>;

    async fn find_by_id(&self, id: i32) -> Result<Option<Person>, anyhow::Error>;
}
