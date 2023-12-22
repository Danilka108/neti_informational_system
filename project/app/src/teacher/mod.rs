use std::collections::HashSet;

use utils::entity::entity;

use crate::{class, person, study_group, subdivision};

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[derive(Debug, Clone)]
#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub person_id: person::EntityId,
    pub kind: TeacherKind,
    pub department_id: subdivision::EntityId,
    pub classes: HashSet<TeacherClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TeacherClass {
    pub study_group_id: study_group::EntityId,
    pub class_id: class::EntityId,
}

#[derive(Debug, Clone)]
pub enum TeacherKind {
    Assistant,
    RegularTeacher,
    SeniorTeacher,
    AssociateProfessor,
    Professor,
}
