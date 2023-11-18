use std::sync::Arc;

use anyhow::Context;
use app::{ports::AccessTokenEngine, token::Claims};
use jsonwebtoken::{Algorithm, Header, Validation};

use super::jwt_claims::JwtClaims;
use super::jwt_keys::JwtKeys;

const JWT_ALGORITHM: Algorithm = Algorithm::HS256;

pub struct JwtAccessTokenEngine {
    pub(crate) keys: Arc<JwtKeys>,
}

#[async_trait::async_trait]
impl AccessTokenEngine for JwtAccessTokenEngine {
    async fn encode(&self, claims: Claims) -> Result<String, anyhow::Error> {
        let mut header = Header::default();
        header.alg = JWT_ALGORITHM;

        jsonwebtoken::encode::<JwtClaims>(&header, &From::from(claims), &self.keys.0)
            .context("failed to encode jwt claims")
    }

    async fn decode(&self, token: &str) -> Result<Claims, anyhow::Error> {
        jsonwebtoken::decode::<JwtClaims>(token, &self.keys.1, &Validation::new(JWT_ALGORITHM))?
            .claims
            .try_into()
            .context("failed to convert raw jwt claims to claims")
    }
}
