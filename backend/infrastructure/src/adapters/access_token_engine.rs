use std::sync::Arc;

use app::{ports::AccessTokenEngine, token::Claims};
use jsonwebtoken::{Algorithm, Header, Validation};

const JWT_ALGORITHM: Algorithm = Algorithm::HS256;

#[derive(Clone)]
pub struct JwtKeys(pub jsonwebtoken::EncodingKey, pub jsonwebtoken::DecodingKey);

impl std::fmt::Debug for JwtKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("JwtKeys").field(&"?").finish()
    }
}

pub struct JwtAccessTokenEngine {
    pub(crate) keys: Arc<JwtKeys>,
}

#[async_trait::async_trait]
impl AccessTokenEngine for JwtAccessTokenEngine {
    async fn encode(&self, claims: Claims) -> Result<String, anyhow::Error> {
        let mut header = Header::default();
        header.alg = JWT_ALGORITHM;
        let token = jsonwebtoken::encode(&header, &claims, &self.keys.0)?;

        Ok(token)
    }

    async fn decode(&self, token: &str) -> Result<Claims, anyhow::Error> {
        let claims =
            jsonwebtoken::decode::<Claims>(token, &self.keys.1, &Validation::new(JWT_ALGORITHM))?;

        Ok(claims.claims)
    }
}
