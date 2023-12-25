use crate::{curriculum, discipline};

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_discipline_id(
        &self,
        discipline_id: discipline::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;

    async fn list_by_curriculum_id(
        &self,
        curriculum_id: curriculum::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;
}
