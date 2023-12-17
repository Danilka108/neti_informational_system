mod access_token_engine;
mod refresh_token_generator;

pub use access_token_engine::AccessTokenEngine;
pub use refresh_token_generator::RefreshTokenGenerator;

use crate::user::{Seconds, SecondsFromUnixEpoch};

pub type BoxedAccessTokenEngine = Box<dyn AccessTokenEngine + Send + Sync>;
pub type BoxedRefreshTokenGenerator = Box<dyn RefreshTokenGenerator + Send + Sync>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
    pub expires_at: SecondsFromUnixEpoch,
    // pub role: Role,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Copy)]
pub struct AccessTokenTTL(pub Seconds);
