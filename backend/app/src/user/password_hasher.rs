use super::HashedPassword;

#[async_trait::async_trait]
pub trait PasswordHasher {
    async fn hash(&self, password: String) -> Result<HashedPassword, anyhow::Error>;

    async fn is_matches(
        &self,
        password: &str,
        hashed_password: &HashedPassword,
    ) -> Result<bool, anyhow::Error>;
}
