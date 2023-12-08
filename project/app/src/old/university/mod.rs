pub mod exceptions;
mod repo;
mod service;

pub use service::UniversityService;
pub type BoxedUniversityRepository = Box<dyn UniversityRepository>;
use crate::SerialId;

pub use self::repo::UniversityRepository;

#[derive(Debug, Clone)]
pub struct University<I = SerialId> {
    pub id: I,
    pub name: String,
}

impl PartialEq for University {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for University {}
