use anyhow::Context;

use super::{AccessTokenTTL, Claims, DynAccessTokenEngine, DynRefreshTokenGenerator, Tokens};
use crate::{session::SecondsFromUnixEpoch, user::User, Outcome};

pub struct TokenService {
    pub(crate) access_token_ttl: AccessTokenTTL,
    pub(crate) access_token_engine: DynAccessTokenEngine,
    pub(crate) refresh_token_generator: DynRefreshTokenGenerator,
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum ExtractClaimsException {
    #[error("expired access token")]
    ExpiredToken,
    #[error("invalid access token")]
    InvalidToken,
}

impl TokenService {
    pub async fn extract_claims(
        self,
        access_token: &str,
    ) -> Outcome<Claims, ExtractClaimsException> {
        let Ok(claims) = self.access_token_engine.decode(access_token).await else {
            return Outcome::Exception(ExtractClaimsException::InvalidToken);
        };

        if claims.expires_at.is_expired()? {
            return Outcome::Exception(ExtractClaimsException::ExpiredToken);
        }

        Outcome::Success(claims)
    }

    pub(crate) async fn generate(self, user: &User) -> Result<Tokens, anyhow::Error> {
        let AccessTokenTTL(ttl) = self.access_token_ttl;

        let expires_at =
            SecondsFromUnixEpoch::from_ttl(ttl).context("failed to generate new expires at")?;

        let claims = Claims {
            user_id: *user.id,
            email: user.email.to_owned(),
            expires_at,
            role: user.role,
        };

        let access_token = self
            .access_token_engine
            .encode(claims)
            .await
            .context("failed to encode jwt token")?;

        let refresh_token = self
            .refresh_token_generator
            .generate()
            .await
            .context("failed to generate refresh toekn")?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }
}
