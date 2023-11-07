use anyhow::Context;

use crate::Outcome;

use super::{DynPersonRepository, Person};

#[derive(Debug, thiserror::Error)]
pub enum CreatePersonException {}

pub struct PersonService {
    pub(crate) repo: DynPersonRepository,
}

impl PersonService {
    pub(crate) async fn create(self) -> Outcome<Person, CreatePersonException> {
        let person = self
            .repo
            .insert(Person { id: () })
            .await?
            .context("currently, inserting a person entity should not result in an 'EntityDoesNotExist' error")?;

        Outcome::Success(person)
    }
}
