use crate::person;

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&mut self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_person_id(
        &mut self,
        person_id: person::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;
}
