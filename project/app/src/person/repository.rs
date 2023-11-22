use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome, SerialId,
};

use super::Person;

#[async_trait::async_trait]
pub trait PersonRepository {
    async fn insert(&self, person: Person<()>) -> Outcome<Person, EntityAlreadyExistError>;

    async fn update(&self, person: Person) -> Outcome<Person, EntityDoesNotExistError>;

    async fn find_by_id(&self, id: SerialId) -> Outcome<Person, EntityNotFoundError>;
}
