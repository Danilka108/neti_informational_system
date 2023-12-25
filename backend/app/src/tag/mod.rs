use utils::entity::entity;

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
#[derive(Clone)]
pub struct Entity {
    #[id]
    pub name: String,
}
