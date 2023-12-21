use super::Entity;

#[async_trait::async_trait]
pub trait Repo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error>;

    async fn delete(&mut self, id: i32) -> Result<Entity, anyhow::Error>;

    async fn find(&mut self, id: i32) -> Result<Option<Entity>, anyhow::Error>;

    async fn find_by_name(&mut self, name: String) -> Result<Option<Entity>, anyhow::Error>;
}
