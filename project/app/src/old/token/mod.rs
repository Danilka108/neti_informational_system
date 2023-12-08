mod access_token_engine;
mod refresh_token_generator;
mod service;

use crate::session::{Seconds, SecondsFromUnixEpoch};
use crate::user::Role;
use crate::SerialId;

pub use access_token_engine::AccessTokenEngine;
pub use refresh_token_generator::RefreshTokenGenerator;
pub use service::{ExtractClaimsException, TokenService};

pub type DynAccessTokenEngine = Box<dyn AccessTokenEngine + Send + Sync>;
pub type DynRefreshTokenGenerator = Box<dyn RefreshTokenGenerator + Send + Sync>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Claims {
    pub user_id: SerialId,
    pub email: String,
    pub expires_at: SecondsFromUnixEpoch,
    pub role: Role,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Copy)]
pub struct AccessTokenTTL(pub Seconds);
