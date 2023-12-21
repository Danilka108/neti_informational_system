use utils::entity::entity;

use crate::{person, subdivision};

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub person_id: person::EntityId,
    pub kind: TeacherKind,
    pub department_id: subdivision::EntityId,
}

pub enum TeacherKind {
    Assistant,
    RegularTeacher,
    SeniorTeacher,
    AssociateProfessor,
    Professor,
}
