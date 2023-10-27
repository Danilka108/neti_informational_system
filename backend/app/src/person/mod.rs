mod repository;
mod service;

use serde::{Deserialize, Serialize};

pub use repository::PersonRepository;
pub use service::{CreatePersonError, PersonService};
pub type DynPersonRepository = Box<dyn PersonRepository + Send + Sync>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Person<Id = i32> {
    pub id: Id,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Person {}
