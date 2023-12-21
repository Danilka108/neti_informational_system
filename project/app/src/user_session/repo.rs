use crate::user;

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&mut self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_user_id(&self, user_id: user::EntityId) -> Result<Vec<Entity>, anyhow::Error>;

    async fn count_not_expired(&self, user_id: user::EntityId) -> Result<i64, anyhow::Error>;
}
