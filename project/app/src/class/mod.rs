use utils::entity::entity;

use crate::{class_kind, curriculum_module};

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub curriculum_module_id: curriculum_module::EntityId,
    pub kind_name: class_kind::EntityId,
}
