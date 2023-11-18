use app::ports::RefreshTokenGenerator;

#[derive(Debug, Clone, Copy)]
pub struct RefreshTokenLength(pub usize);

pub struct NanoIdRefreshTokenGenerator {
    pub(crate) length: RefreshTokenLength,
}

#[async_trait::async_trait]
impl RefreshTokenGenerator for NanoIdRefreshTokenGenerator {
    async fn generate(&self) -> Result<String, anyhow::Error> {
        let length = self.length.0;
        Ok(nanoid::nanoid!(length))
    }
}
