pub mod exceptions;
mod repositories;
pub mod service;

use std::num::NonZeroI32;

use crate::{person::Person, tag::Tag, university::University};

pub use repositories::SubdivisionRepository;
pub type BoxedSubdivisionRepository = Box<dyn SubdivisionRepository>;
pub type BoxedSubdivisionMemberRepository = Box<dyn SubdivisionMemberRepository>;
pub type BoxedSubdivisionTagRepository = Box<dyn SubdivisionTagRepository>;
pub use self::repositories::{SubdivisionMemberRepository, SubdivisionTagRepository};

#[derive(Debug, Clone)]
pub struct Subdivision<Id = NonZeroI32> {
    pub id: Id,
    pub name: String,
    pub university: University,
}

impl PartialEq for Subdivision {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Subdivision {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubdivisionTag(pub Subdivision, pub Tag);

#[derive(Debug, Clone)]
pub struct SubdivisionMember {
    pub id: (Subdivision, Person),
    pub role: String,
}

impl PartialEq for SubdivisionMember {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for SubdivisionMember {}
