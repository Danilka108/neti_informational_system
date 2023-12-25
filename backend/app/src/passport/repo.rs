use crate::person;

use super::{Entity, EntityId, PassportNumber, PassportSeries};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn find_by_number_series(
        &self,
        number: PassportNumber,
        series: PassportSeries,
    ) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_person_id(
        &self,
        person_id: person::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;
}
