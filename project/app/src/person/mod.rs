mod repository;
mod service;

use std::num::NonZeroI32;

pub use repository::PersonRepository;
pub use service::{
    CreatePersonException, GetPersonException, PersonDoesNotExistError, PersonService,
};
pub type DynPersonRepository = Box<dyn PersonRepository + Send + Sync>;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Person<Id = NonZeroI32> {
    pub id: Id,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Person {}
