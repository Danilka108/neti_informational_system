mod repository;
mod service;

pub use repository::PersonRepository;
pub use service::{
    CreatePersonException, GetPersonException, PersonDoesNotExistError, PersonService,
};

use crate::SerialId;
pub type DynPersonRepository = Box<dyn PersonRepository + Send + Sync>;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Person<Id = SerialId> {
    pub id: Id,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Person {}
