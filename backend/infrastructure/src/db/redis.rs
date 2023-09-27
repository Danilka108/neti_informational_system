use crate::env_config::EnvConfig;

pub async fn init_redis_conn_pool(
    env_config: &EnvConfig,
) -> Result<mobc::Pool<RedisConnectionManager>, redis::RedisError> {
    let client = redis::Client::open(&env_config.redis_uri[..])?;
    let manager = RedisConnectionManager { client };

    Ok(mobc::Pool::new(manager))
}

#[derive(Debug)]
pub struct RedisConnectionManager {
    client: redis::Client,
}

#[async_trait::async_trait]
impl mobc::Manager for RedisConnectionManager {
    type Connection = redis::aio::Connection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_tokio_connection().await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        redis::cmd("PING").query_async(&mut conn).await?;
        Ok(conn)
    }
}
