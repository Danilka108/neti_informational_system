use std::{str::FromStr, sync::Arc};

use serde::{de::Error, de::Unexpected, Deserialize, Deserializer};

#[derive(Debug, serde::Deserialize)]
pub struct EnvConfig {
    #[serde(default = "get_default_workers_count")]
    pub pg_max_connections: u32,
    pub pg_dbname: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_host: String,
    pub pg_port: u16,
    pub sqlx_log: Option<log::LevelFilter>,

    pub refresh_token_length: usize,

    pub jwt_secret: String,
    pub jwt_token_ttl_in_seconds: u64,

    pub sessions_max_number_per_user: usize,
    pub session_ttl_in_seconds: u64,

    #[serde(default = "get_default_workers_count")]
    pub argon2_parallelism_degree: u32,
    #[serde(deserialize_with = "deserialize_argon2_algorithm")]
    pub argon2_algorithm: argon2::Algorithm,
    #[serde(deserialize_with = "deserialize_argon2_version")]
    pub argon2_version: argon2::Version,

    #[serde(default = "get_default_workers_count")]
    pub workers_count: u32,

    pub application_name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("failed to load env config, cause {}", .0)]
pub struct LoadEnvConfigError(#[from] envy::Error);

impl EnvConfig {
    pub fn try_load() -> Result<Self, LoadEnvConfigError> {
        envy::from_env().map_err(LoadEnvConfigError)
    }
}

fn get_default_workers_count() -> u32 {
    std::thread::available_parallelism().unwrap().get() as u32
}

fn deserialize_argon2_version<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<argon2::Version, D::Error> {
    let value = u32::deserialize(deserializer)?;

    match argon2::Version::try_from(value) {
        Err(_) => {
            let unexpected = Unexpected::Unsigned(value as u64);
            Err(D::Error::invalid_value(unexpected, &"0x10 or 0x13"))
        }
        Ok(v) => Ok(v),
    }
}

fn deserialize_argon2_algorithm<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<argon2::Algorithm, D::Error> {
    let value = String::deserialize(deserializer)?;

    match argon2::Algorithm::from_str(&value) {
        Err(err) => {
            let unexpected = Unexpected::Str(&value);

            // TODO should be changed
            let err = err.to_string();
            let expected = err.as_str();

            Err(D::Error::invalid_value(unexpected, &expected))
        }
        Ok(v) => Ok(v),
    }
}

impl Into<super::ConfigModule> for EnvConfig {
    fn into(self) -> super::ConfigModule {
        use adapters::config::*;
        use app::session::Seconds;

        // TODO
        // maybe remove as_bytes should be removed
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_bytes());
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let jwt_keys = Arc::new(JwtKeys(encoding_key, decoding_key));

        super::ConfigModule {
            sessions_max_number: SessionsMaxNumber(self.sessions_max_number_per_user),
            session_ttl: SessionTTL(Seconds::from(self.session_ttl_in_seconds)),
            access_token_ttl: AccessTokenTTL(Seconds::from(self.jwt_token_ttl_in_seconds)),
            refresh_token_length: RefreshTokenLength(self.refresh_token_length),
            argon2_params: Argon2Params {
                paralelism_degree: self.argon2_parallelism_degree,
                version: self.argon2_version,
                algorithm: self.argon2_algorithm,
            },
            jwt_keys,
            pg_host: Arc::from(self.pg_host),
            pg_password: Arc::from(self.pg_password),
            pg_database_name: Arc::from(self.pg_dbname),
            application_name: Arc::from(self.application_name),
            pg_port: self.pg_port,
            pg_user_name: Arc::from(self.pg_username),
            sqlx_log_level_filter: self.sqlx_log,
            sqlx_max_connections: self.pg_max_connections,
        }
    }
}
