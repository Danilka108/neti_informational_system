use std::num::NonZeroI32;

use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome,
};

use super::University;

#[async_trait::async_trait]
pub trait UniversityRepository {
    async fn insert(
        &self,
        university: University<()>,
    ) -> Outcome<University, EntityAlreadyExistError>;

    async fn delete(&self, id: NonZeroI32) -> Outcome<University, EntityDoesNotExistError>;

    async fn get(&self, id: NonZeroI32) -> Outcome<University, EntityNotFoundError>;
}
