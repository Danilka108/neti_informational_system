use std::num::NonZeroU32;

#[derive(Debug, serde::Deserialize)]
pub struct EnvConfig {
    pub application_name: Box<str>,
    pub jwt_secret: Box<str>,
    pub pg_dbname: Box<str>,
    pub pg_username: Box<str>,
    pub pg_password: Box<str>,
    pub pg_host: Box<str>,
    pub pg_port: u16,
    #[serde(default = "get_default_workers_count")]
    pub pg_max_connections: u32,
    #[serde(default = "get_default_workers_count")]
    pub workers_count: u32,
    pub sqlx_log: Option<log::LevelFilter>,
    pub redis_uri: String,
    pub redis_max_connections: u32,
    pub pbkdf2_iters_count: NonZeroU32,
    pub pbkdf2_salt: Box<[u8]>,
}

fn get_default_workers_count() -> u32 {
    std::thread::available_parallelism().unwrap().get() as u32
}

impl EnvConfig {
    pub fn try_load() -> Result<Self, LoadEnvConfigError> {
        envy::from_env().map_err(LoadEnvConfigError)
    }
}

#[derive(Debug)]
pub struct LoadEnvConfigError(envy::Error);

impl std::fmt::Display for LoadEnvConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to load env config, cause: {}", self.0)
    }
}

impl std::error::Error for LoadEnvConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
