use utils::entity::entity;

use crate::{curriculum_module, study_group, teacher};

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub curriculum_module_id: curriculum_module::EntityId,
    pub kind: Box<str>,
    pub teachers: Box<[(teacher::EntityId, study_group::EntityId)]>,
}
