use std::sync::Arc;

use crate::env_config::EnvConfig;
use adapters::config::{Argon2Params, JwtKeys, RefreshTokenLength};
use app::{
    session::{Seconds, SessionTTL, SessionsMaxNumber},
    token::AccessTokenTTL,
};
use di::{Module, Provide};

#[derive(Clone, Debug)]
pub struct ConfigContainer {
    sessions_max_number: SessionsMaxNumber,
    session_ttl: SessionTTL,
    access_token_ttl: AccessTokenTTL,
    refresh_token_length: RefreshTokenLength,
    argon2_params: Argon2Params,
    jwt_keys: Arc<adapters::config::JwtKeys>,
}

impl<'c> From<&'c EnvConfig> for ConfigContainer {
    fn from(env_config: &'c EnvConfig) -> Self {
        // TODO
        // maybe remove as_bytes should be removed
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(env_config.jwt_secret.as_bytes());
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(env_config.jwt_secret.as_bytes());
        let jwt_keys = Arc::new(JwtKeys(encoding_key, decoding_key));

        Self {
            sessions_max_number: SessionsMaxNumber(env_config.sessions_max_number_per_user),
            session_ttl: SessionTTL(Seconds::from(env_config.session_ttl_in_seconds)),
            access_token_ttl: AccessTokenTTL(Seconds::from(env_config.jwt_token_ttl_in_seconds)),
            refresh_token_length: RefreshTokenLength(env_config.refresh_token_length),
            argon2_params: Argon2Params {
                paralelism_degree: env_config.argon2_parallelism_degree,
                version: env_config.argon2_version,
                algorithm: env_config.argon2_algorithm,
            },
            jwt_keys,
        }
    }
}

impl Module for ConfigContainer {}

impl adapters::ConfigModule for ConfigContainer {}

impl Provide<adapters::config::SessionsMaxNumber> for ConfigContainer {
    fn provide(&self) -> SessionsMaxNumber {
        self.sessions_max_number
    }
}

impl Provide<adapters::config::SessionTTL> for ConfigContainer {
    fn provide(&self) -> SessionTTL {
        self.session_ttl
    }
}

impl Provide<Arc<adapters::config::JwtKeys>> for ConfigContainer {
    fn provide(&self) -> Arc<adapters::config::JwtKeys> {
        Arc::clone(&self.jwt_keys)
    }
}

impl Provide<adapters::config::AccessTokenTTL> for ConfigContainer {
    fn provide(&self) -> adapters::config::AccessTokenTTL {
        self.access_token_ttl
    }
}

impl Provide<adapters::config::RefreshTokenLength> for ConfigContainer {
    fn provide(&self) -> adapters::config::RefreshTokenLength {
        self.refresh_token_length
    }
}

impl Provide<adapters::config::Argon2Params> for ConfigContainer {
    fn provide(&self) -> adapters::config::Argon2Params {
        self.argon2_params
    }
}
