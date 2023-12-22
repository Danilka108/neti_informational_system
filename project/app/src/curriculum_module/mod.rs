use utils::entity::entity;

use crate::{curriculum, discipline};

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub discipline_id: discipline::EntityId,
    pub curriculum_id: curriculum::EntityId,
    pub semester: i32,
}
