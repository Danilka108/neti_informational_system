use app::{
    session::{Seconds, SecondsFromUnixEpoch},
    token::Claims,
    user::Role,
    SerialId,
};
use serde::{Deserialize, Serialize};

const ADMIN_ROLE_IDENT: &str = "ADMIN";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JwtClaims {
    pub user_id: SerialId,
    pub email: String,
    pub expires_at: u64,
    pub role: String,
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
            role: match &value.role[..] {
                ADMIN_ROLE_IDENT => Role::Admin,
                _ => return Err(ConvertJwtClaimsError),
            },
        })
    }
}

impl From<Claims> for JwtClaims {
    fn from(value: Claims) -> Self {
        Self {
            user_id: value.user_id,
            email: value.email,
            expires_at: value.expires_at.seconds.val,
            role: match value.role {
                Role::Admin => ADMIN_ROLE_IDENT.to_owned(),
            },
        }
    }
}
