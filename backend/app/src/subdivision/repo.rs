use std::collections::HashSet;

use crate::{person, tag, university};

use super::{Entity, EntityId};

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error>;

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error>;

    async fn find_by_name(&self, name: String) -> Result<Option<Entity>, anyhow::Error>;

    async fn list_by_university(
        &self,
        university_id: university::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error>;

    async fn list_by_tags(
        &self,
        tags_ids: HashSet<tag::EntityId>,
    ) -> Result<Vec<Entity>, anyhow::Error>;

    async fn list_by_members(
        &self,
        persons_ids: HashSet<person::EntityId>,
    ) -> Result<Vec<Entity>, anyhow::Error>;
}
