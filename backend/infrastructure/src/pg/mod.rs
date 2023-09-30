mod transaction;
mod user_repository;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

use crate::env_config::EnvConfig;

pub async fn init_pg_conn_pool(env_config: &EnvConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    let conn_options = PgConnectOptions::new()
        .host(&env_config.pg_host)
        .port(env_config.pg_port)
        .username(&env_config.pg_username)
        .password(&env_config.pg_password)
        .database(&env_config.pg_dbname)
        .application_name(&env_config.application_name);

    let conn_options = if let Some(level_filter) = env_config.sqlx_log {
        conn_options.log_statements(level_filter)
    } else {
        conn_options
    };

    PgPoolOptions::new()
        .max_connections(env_config.pg_max_connections)
        .connect_with(conn_options)
        .await
}
