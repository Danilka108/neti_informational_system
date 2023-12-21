use std::num::NonZeroU8;

use utils::entity::entity;

use crate::{curriculum, discipline};

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub discipline_id: discipline::EntityId,
    pub curriculum_id: curriculum::EntityId,
    pub semester: NonZeroU8,
}
