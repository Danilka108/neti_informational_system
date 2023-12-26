use crate::user;

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn list(&self) -> Result<Vec<Entity>, anyhow::Error>;

    async fn find_by_user_id(
        &self,
        user_id: user::EntityId,
    ) -> Result<Option<Entity>, anyhow::Error>;
}
