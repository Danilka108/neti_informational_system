use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome, SerialId,
};

use super::University;

#[async_trait::async_trait]
pub trait UniversityRepository {
    async fn insert(
        &self,
        university: University<()>,
    ) -> Outcome<University, EntityAlreadyExistError>;

    async fn delete(&self, id: SerialId) -> Outcome<University, EntityDoesNotExistError>;

    async fn get(&self, id: SerialId) -> Outcome<University, EntityNotFoundError>;
}
