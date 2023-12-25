use utils::entity::entity;

use crate::subdivision;

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
    pub department_id: subdivision::EntityId,
}
