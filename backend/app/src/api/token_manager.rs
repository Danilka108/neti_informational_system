pub trait TokenManager {
    fn encode_access_token(&self, claims: domain::AuthClaims) -> Result<String, anyhow::Error>;

    fn decode_access_token(&self, token: &str) -> Result<domain::AuthClaims, anyhow::Error>;

    fn generate_refresh_token(&self) -> Result<String, anyhow::Error>;
}
