mod repo;

use std::collections::HashSet;

use crate::{person, tag, university};
use utils::entity::entity;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
#[derive(Debug, Clone)]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
    pub university_id: university::EntityId,
    pub tags: HashSet<tag::EntityId>,
    pub members: HashSet<Member>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Member {
    pub person_id: person::EntityId,
    pub role: String,
}
