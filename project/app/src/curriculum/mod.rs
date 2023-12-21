use utils::entity::entity;

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
}
