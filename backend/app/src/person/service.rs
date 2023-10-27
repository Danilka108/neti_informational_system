use anyhow::Context;

use super::{DynPersonRepository, Person};

pub struct PersonService {
    pub(crate) repo: DynPersonRepository,
}

#[derive(Debug, thiserror::Error)]
pub enum CreatePersonError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl PersonService {
    pub(crate) async fn create(self) -> Result<Person, CreatePersonError> {
        let person = self
            .repo
            .insert(Person { id: () })
            .await?
            .context("currently, inserting a person entity should not result in an 'EntityDoesNotExist' error")?;

        Ok(person)
    }
}
