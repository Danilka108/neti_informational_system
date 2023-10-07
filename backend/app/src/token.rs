use anyhow::Context;
use domain::{AuthClaims, Seconds, SecondsFromUnixEpoch, User};

use crate::{api::TokenManager, dyn_dependency};

pub struct TokenService {
    token_manager: dyn_dependency!(TokenManager),
    access_token_ttl: Seconds,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateAccessTokenError {
    #[error("expired token")]
    ExpiredToken,
    #[error("invalid token")]
    InvalidToken,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl TokenService {
    pub fn new(access_token_ttl: Seconds, token_manager: dyn_dependency!(TokenManager)) -> Self {
        Self {
            token_manager,
            access_token_ttl,
        }
    }

    pub fn validate_access_token(
        &self,
        access_token: &str,
    ) -> Result<AuthClaims, ValidateAccessTokenError> {
        let Ok(claims) = self.token_manager.decode_access_token(access_token) else {
            return Err(ValidateAccessTokenError::InvalidToken);
        };

        if claims.expires_at.is_expired()? {
            return Err(ValidateAccessTokenError::ExpiredToken);
        }

        Ok(claims)
    }

    pub fn generate_access_token(&self, user: &User) -> Result<String, anyhow::Error> {
        let claims = AuthClaims {
            user_id: user.id,
            expires_at: SecondsFromUnixEpoch::new_expires_at(self.access_token_ttl)
                .context("failed to generate new expires at")?,
            role: user.role,
            email: user.email.clone(),
        };

        self.token_manager
            .encode_access_token(claims)
            .context("failed to encode jwt token")
    }

    pub fn generate_refresh_token(&self) -> Result<String, anyhow::Error> {
        self.token_manager.generate_refresh_token()
    }
}
