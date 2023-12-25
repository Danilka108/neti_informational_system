mod repo;
mod seconds;

use utils::entity::entity;

use crate::user;

// pub use ex::*;
pub use repo::Repo;
pub use seconds::*;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[derive(Debug, Clone)]
#[entity]
pub struct Entity {
    #[id]
    pub id: Id,
    pub refresh_token: String,
    pub expires_at: SecondsFromUnixEpoch,
}

#[derive(Debug, Clone)]
pub struct Id {
    pub user_id: user::EntityId,
    pub metadata: String,
}

#[derive(Debug, Clone, Copy)]
pub struct SessionTTL(pub Seconds);

#[derive(Debug, Clone, Copy)]
pub struct SessionsMaxNumber(pub i64);
