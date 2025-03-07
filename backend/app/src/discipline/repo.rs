use crate::subdivision;

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn find_by_name(&self, name: String) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_department_id(
        &self,
        department_id: subdivision::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;
}
