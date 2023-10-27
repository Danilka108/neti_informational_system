use super::Claims;

#[async_trait::async_trait]
pub trait AccessTokenEngine {
    async fn encode(&self, claims: Claims) -> Result<String, anyhow::Error>;

    async fn decode(&self, token: &str) -> Result<Claims, anyhow::Error>;
}
