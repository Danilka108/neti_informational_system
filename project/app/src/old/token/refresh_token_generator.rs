#[async_trait::async_trait]
pub trait RefreshTokenGenerator {
    async fn generate(&self) -> Result<String, anyhow::Error>;
}
