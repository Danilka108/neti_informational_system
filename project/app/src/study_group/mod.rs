mod repo;

use crate::{curriculum, subdivision};

pub use repo::Repo;
use utils::entity::entity;

pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
    pub studying_qualification: Qualification,
    pub training_kind: TrainingKind,
    pub department_id: subdivision::EntityId,
    pub curriculums: Vec<curriculum::EntityId>,
}

pub enum Qualification {
    Bachelor,
    Master,
    Postgraduate,
    Doctorate,
}

pub enum TrainingKind {
    FullTime,
    Correspondence,
}
