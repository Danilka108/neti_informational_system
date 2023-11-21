use std::num::NonZeroI32;

use crate::{ports::EntityNotFoundError, Outcome};

use super::{DynPersonRepository, Person};

#[derive(Debug, thiserror::Error)]
pub enum CreatePersonException {}

#[derive(Debug, thiserror::Error)]
#[error("person {id} does not exist")]
pub struct PersonDoesNotExistError {
    id: NonZeroI32,
}

#[derive(Debug, thiserror::Error)]
pub enum GetPersonException {
    #[error(transparent)]
    PersonDoesNotExist(#[from] PersonDoesNotExistError),
}

pub struct PersonService {
    pub(crate) repo: DynPersonRepository,
}

impl PersonService {
    pub(crate) async fn create(self) -> Outcome<Person, CreatePersonException> {
        match self.repo.insert(Person { id: () }).await {
            Outcome::Success(person) => Outcome::Success(person),
            Outcome::Exception(_) => panic!("currently, inserting a person entity should not result in an 'EntityAlreadyExistError' error"),
            Outcome::Unexpected(err) => Outcome::Unexpected(err)
        }
    }

    pub async fn get(self, id: NonZeroI32) -> Outcome<Person, GetPersonException> {
        let person = self
            .repo
            .find_by_id(id)
            .await
            .map_exception(|EntityNotFoundError| PersonDoesNotExistError { id })?;

        Outcome::Success(person)
    }
}
