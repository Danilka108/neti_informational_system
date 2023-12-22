mod repo;

use std::collections::HashSet;

use crate::{curriculum, subdivision};

pub use repo::Repo;
use utils::entity::entity;

pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[derive(Debug, Clone)]
#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
    pub studying_qualification: Qualification,
    pub training_kind: TrainingKind,
    pub department_id: subdivision::EntityId,
    pub curriculums: HashSet<curriculum::EntityId>,
}

#[derive(Debug, Clone)]
pub enum Qualification {
    Bachelor,
    Master,
    Postgraduate,
    Doctorate,
}

#[derive(Debug, Clone)]
pub enum TrainingKind {
    FullTime,
    Correspondence,
}
