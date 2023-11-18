mod repo;

use std::num::NonZeroI32;

pub use repo::TagRepository;

pub type BoxedTagRepository = Box<dyn TagRepository>;

#[derive(Debug, Clone)]
pub struct Tag<Id = NonZeroI32> {
    pub id: Id,
    pub name: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Tag {}
