pub mod env_config;

use adapters::config::*;
use app::{
    session::{SessionTTL, SessionsMaxNumber},
    token::AccessTokenTTL,
};
use di::{Module, Provide};
use log::LevelFilter;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConfigModule {
    pub sessions_max_number: SessionsMaxNumber,
    pub session_ttl: SessionTTL,
    pub access_token_ttl: AccessTokenTTL,
    pub refresh_token_length: RefreshTokenLength,
    pub argon2_params: Argon2Params,
    pub jwt_keys: Arc<JwtKeys>,
    pub pg_host: Arc<str>,
    pub pg_port: u16,
    pub pg_user_name: Arc<str>,
    pub pg_password: Arc<str>,
    pub pg_database_name: Arc<str>,
    pub application_name: Arc<str>,
    pub sqlx_log_level_filter: Option<LevelFilter>,
    pub sqlx_max_connections: u32,
}

impl Module for ConfigModule {}

impl adapters::config::ConfigModule for ConfigModule {}

impl Provide<SessionsMaxNumber> for ConfigModule {
    fn provide(&self) -> SessionsMaxNumber {
        self.sessions_max_number
    }
}

impl Provide<SessionTTL> for ConfigModule {
    fn provide(&self) -> SessionTTL {
        self.session_ttl
    }
}

impl Provide<Arc<JwtKeys>> for ConfigModule {
    fn provide(&self) -> Arc<JwtKeys> {
        Arc::clone(&self.jwt_keys)
    }
}

impl Provide<AccessTokenTTL> for ConfigModule {
    fn provide(&self) -> AccessTokenTTL {
        self.access_token_ttl
    }
}

impl Provide<RefreshTokenLength> for ConfigModule {
    fn provide(&self) -> RefreshTokenLength {
        self.refresh_token_length
    }
}

impl Provide<Argon2Params> for ConfigModule {
    fn provide(&self) -> Argon2Params {
        self.argon2_params
    }
}

impl Provide<PgHost> for ConfigModule {
    fn provide(&self) -> PgHost {
        PgHost(Arc::clone(&self.pg_host))
    }
}

impl Provide<PgPort> for ConfigModule {
    fn provide(&self) -> PgPort {
        PgPort(self.pg_port)
    }
}

impl Provide<PgUserName> for ConfigModule {
    fn provide(&self) -> PgUserName {
        PgUserName(Arc::clone(&self.pg_user_name))
    }
}

impl Provide<PgPassword> for ConfigModule {
    fn provide(&self) -> PgPassword {
        PgPassword(Arc::clone(&self.pg_password))
    }
}

impl Provide<PgDatabaseName> for ConfigModule {
    fn provide(&self) -> PgDatabaseName {
        PgDatabaseName(Arc::clone(&self.pg_database_name))
    }
}

impl Provide<ApplicationName> for ConfigModule {
    fn provide(&self) -> ApplicationName {
        ApplicationName(Arc::clone(&self.application_name))
    }
}

impl Provide<SqlxLogLevelFilter> for ConfigModule {
    fn provide(&self) -> SqlxLogLevelFilter {
        SqlxLogLevelFilter(self.sqlx_log_level_filter)
    }
}

impl Provide<SqlxMaxConnections> for ConfigModule {
    fn provide(&self) -> SqlxMaxConnections {
        SqlxMaxConnections(self.sqlx_max_connections)
    }
}

// fn from(env_config: EnvConfig) -> Self {
//     // TODO
//     // maybe remove as_bytes should be removed
//     let encoding_key = jsonwebtoken::EncodingKey::from_secret(env_config.jwt_secret.as_bytes());
//     let decoding_key = jsonwebtoken::DecodingKey::from_secret(env_config.jwt_secret.as_bytes());
//     let jwt_keys = Arc::new(JwtKeys(encoding_key, decoding_key));

//     Self {
//         sessions_max_number: SessionsMaxNumber(env_config.sessions_max_number_per_user),
//         session_ttl: SessionTTL(Seconds::from(env_config.session_ttl_in_seconds)),
//         access_token_ttl: AccessTokenTTL(Seconds::from(env_config.jwt_token_ttl_in_seconds)),
//         refresh_token_length: RefreshTokenLength(env_config.refresh_token_length),
//         argon2_params: Argon2Params {
//             paralelism_degree: env_config.argon2_parallelism_degree,
//             version: env_config.argon2_version,
//             algorithm: env_config.argon2_algorithm,
//         },
//         jwt_keys,
//         pg_host: Arc::from(env_config.pg_host),
//         pg_password: Arc::from(env_config.pg_password),
//         pg_database_name: Arc::from(env_config.pg_dbname),
//         application_name: Arc::from(env_config.application_name),
//         pg_port: env_config.pg_port,
//         pg_user_name: Arc::from(env_config.pg_username),
//         sqlx_log_level_filter: env_config.sqlx_log,
//         sqlx_max_connections: env_config.pg_max_connections,
//     }
// }
