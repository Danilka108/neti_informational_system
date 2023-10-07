use domain::AuthClaims;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::env_config::EnvConfig;

pub struct TokenManagerImpl {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    refresh_token_length: usize,
}

const JWT_ALGORITHM: Algorithm = Algorithm::HS256;

impl TokenManagerImpl {
    pub fn new(config: &EnvConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            refresh_token_length: config.refresh_token_length,
        }
    }
}

impl app::api::TokenManager for TokenManagerImpl {
    fn encode_access_token(&self, claims: domain::AuthClaims) -> Result<String, anyhow::Error> {
        let mut header = Header::default();
        header.alg = JWT_ALGORITHM;
        let token = jsonwebtoken::encode(&header, &claims, &self.encoding_key)?;

        Ok(token)
    }

    fn decode_access_token(&self, token: &str) -> Result<domain::AuthClaims, anyhow::Error> {
        let claims = jsonwebtoken::decode::<AuthClaims>(
            token,
            &self.decoding_key,
            &Validation::new(JWT_ALGORITHM),
        )?;

        Ok(claims.claims)
    }

    fn generate_refresh_token(&self) -> Result<String, anyhow::Error> {
        let length = self.refresh_token_length;
        Ok(nanoid::nanoid!(length))
    }
}
