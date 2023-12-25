mod repo;

use utils::entity::entity;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
#[derive(Clone)]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
}
