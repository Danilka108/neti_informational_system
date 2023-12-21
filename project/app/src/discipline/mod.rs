use utils::entity::entity;

use crate::subdivision;

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: Box<str>,
    pub department_id: subdivision::EntityId,
}
