use std::collections::HashSet;

use utils::entity::entity;

use crate::{curriculum_module, teacher};

mod repo;
pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[derive(Debug, Clone)]
#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub curriculum_module_id: curriculum_module::EntityId,
    pub kind: AttestationKind,
    pub duration: Hours,
    pub examiners: HashSet<teacher::EntityId>,
}

#[derive(Debug, Clone)]
pub struct Score(pub u8);

#[derive(Debug, Clone)]
pub struct Hours(pub i32);

#[derive(Debug, Clone)]
pub enum AttestationKind {
    Test,
    DiffTest,
    Exam,
}
