use super::Tag;

#[async_trait::async_trait]
pub trait TagRepository {
    async fn insert(&self, tag: Tag<()>) -> Result<Tag, anyhow::Error>;

    async fn update(&self, tag: Tag) -> Result<Tag, anyhow::Error>;

    async fn remove(&self, tag: Tag) -> Result<Tag, anyhow::Error>;
}
