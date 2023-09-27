use crate::{
    auth::gen_jwt_keys,
    db::pg::init_pg_conn_pool,
    db::redis::{init_redis_conn_pool, RedisConnectionManager},
    env_config::EnvConfig,
};

pub struct AppState {
    env_config: EnvConfig,
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    redis_pool: mobc::Pool<RedisConnectionManager>,
    jwt_keys: (jsonwebtoken::EncodingKey, jsonwebtoken::DecodingKey),
}

impl AppState {
    pub async fn new(env_config: EnvConfig) -> Result<Self, AppStateError> {
        let pg_pool = init_pg_conn_pool(&env_config)
            .await
            .map_err(AppStateError::InitPgConnPool)?;

        let redis_pool = init_redis_conn_pool(&env_config)
            .await
            .map_err(AppStateError::InitRedisConnPool)?;

        let jwt_keys = gen_jwt_keys(&env_config);

        Ok(Self {
            env_config,
            pg_pool,
            redis_pool,
            jwt_keys,
        })
    }

    pub fn pg_pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.pg_pool
    }

    pub fn redis_pool(&self) -> &mobc::Pool<RedisConnectionManager> {
        &self.redis_pool
    }

    pub fn jwt_decoding_key(&self) -> &jsonwebtoken::DecodingKey {
        &self.jwt_keys.1
    }

    pub fn jwt_encoding_key(&self) -> &jsonwebtoken::EncodingKey {
        &self.jwt_keys.0
    }

    pub fn env_config(&self) -> &EnvConfig {
        &self.env_config
    }
}

#[derive(Debug)]
pub enum AppStateError {
    InitPgConnPool(sqlx::Error),
    InitRedisConnPool(redis::RedisError),
}

impl std::fmt::Display for AppStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitPgConnPool(err) => {
                write!(f, "failed to init postgres connection pool, cause {err}")
            }
            Self::InitRedisConnPool(err) => {
                write!(f, "failed to init redis connecion pool, cause {err}")
            }
        }
    }
}

impl std::error::Error for AppStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InitPgConnPool(err) => Some(err),
            Self::InitRedisConnPool(err) => Some(err),
        }
    }
}
