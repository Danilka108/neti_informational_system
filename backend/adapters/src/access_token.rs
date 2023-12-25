use serde::{Deserialize, Serialize};

use std::sync::Arc;

use anyhow::Context;
use jsonwebtoken::{Algorithm, Header, Validation};

use app::token::{AccessTokenEngine, Claims};
use app::user_session::{Seconds, SecondsFromUnixEpoch};

const JWT_ALGORITHM: Algorithm = Algorithm::HS256;
// const ADMIN_ROLE_IDENT: &str = "ADMIN";

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JwtClaims {
    pub user_id: i32,
    pub email: String,
    pub expires_at: u64,
    // pub role: String,
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

#[derive(Debug, thiserror::Error)]
#[error("failed to convert raw jwt claims into claims")]
pub struct ConvertJwtClaimsError;

impl TryFrom<JwtClaims> for Claims {
    type Error = ConvertJwtClaimsError;

    fn try_from(value: JwtClaims) -> Result<Self, Self::Error> {
        Ok(Claims {
            user_id: value.user_id,
            email: value.email,
            expires_at: SecondsFromUnixEpoch {
                seconds: Seconds {
                    val: value.expires_at,
                },
            },
            // role: match &value.role[..] {
            //     ADMIN_ROLE_IDENT => Role::Admin,
            //     _ => return Err(ConvertJwtClaimsError),
            // },
        })
    }
}

impl From<Claims> for JwtClaims {
    fn from(value: Claims) -> Self {
        Self {
            user_id: value.user_id,
            email: value.email,
            expires_at: value.expires_at.seconds.val,
            // role: match value.role {
            //     Role::Admin => ADMIN_ROLE_IDENT.to_owned(),
            // },
        }
    }
}
