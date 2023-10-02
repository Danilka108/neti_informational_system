pub trait TokenManager {
    type Error: std::error::Error + Send + Sync + 'static;

    fn encode_jwt_token(&self, claims: domain::AuthClaims) -> Result<String, Self::Error>;

    fn decode_jwt_token(&self, token: &str) -> Result<domain::AuthClaims, Self::Error>;

    fn generate_refresh_token(&self) -> Result<String, Self::Error>;
}
