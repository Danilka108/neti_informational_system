use utils::entity::entity;

use crate::{curriculum_module, teacher};

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub curriculum_module_id: curriculum_module::EntityId,
    pub kind: AttestationKind,
    pub duration: Hours,
    pub examiners: Box<[teacher::EntityId]>,
}

pub struct Score(pub u8);

pub struct Hours(pub u8);

pub enum AttestationKind {
    Test,
    DiffTest,
    Exam,
}
