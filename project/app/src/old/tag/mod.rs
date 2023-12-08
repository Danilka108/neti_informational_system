pub mod exceptions;
mod repo;
mod service;

pub use repo::TagRepository;
pub use service::TagService;

use crate::SerialId;

pub type BoxedTagRepository = Box<dyn TagRepository>;

#[derive(Debug, Clone)]
pub struct Tag<Id = SerialId> {
    pub id: Id,
    pub name: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Tag {}
