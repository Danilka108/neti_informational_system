mod repo;

pub use repo::Repo;

use crate::user;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[utils::entity::entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub user_id: user::EntityId,
    pub full_name: String,
}
