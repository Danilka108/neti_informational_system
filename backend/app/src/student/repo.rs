use std::collections::HashSet;

use crate::{attestation, person, study_group};

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_person(
        &self,
        person_id: person::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;

    async fn list_by_study_group(
        &self,
        study_group_id: study_group::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;

    async fn list_by_attestations(
        &self,
        attestations_ids: HashSet<attestation::EntityId>,
    ) -> Result<Vec<Entity>, anyhow::Error>;
}
