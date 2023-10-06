use anyhow::Context;
use domain::{AuthClaims, Seconds, SecondsFromUnixEpoch, User};

use crate::api::TokenManager;

pub struct TokenService {
    token_manager: Box<dyn TokenManager>,
}

impl TokenService {
    pub fn new(token_manager: Box<dyn TokenManager>) -> Self {
        Self { token_manager }
    }

    pub fn generate_access_token(
        &self,
        user: &User,
        ttl: Seconds,
    ) -> Result<String, anyhow::Error> {
        let claims = AuthClaims {
            user_id: user.id,
            expires_at: SecondsFromUnixEpoch::new_expires_at(ttl)
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
