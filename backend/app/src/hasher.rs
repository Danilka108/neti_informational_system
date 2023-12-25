#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPassword {
    pub value: String,
}

pub type BoxedPasswordHasher = Box<dyn PasswordHasher + Send + Sync>;

#[async_trait::async_trait]
pub trait PasswordHasher {
    async fn hash(&self, password: String) -> Result<HashedPassword, anyhow::Error>;

    async fn is_matches(
        &self,
        password: &str,
        hashed_password: &HashedPassword,
    ) -> Result<bool, anyhow::Error>;
}
