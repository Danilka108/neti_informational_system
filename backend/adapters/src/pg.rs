use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

use crate::config::{
    ApplicationName, ConfigModule, PgDatabaseName, PgHost, PgPassword, PgPort, PgUserName,
    SqlxLogLevelFilter, SqlxMaxConnections,
};

pub async fn init_pg_conn_pool<C: ConfigModule>(cfg: &C) -> Result<Pool<Postgres>, sqlx::Error> {
    let conn_options = PgConnectOptions::new()
        .host(&cfg.resolve::<PgHost>().0)
        .port(cfg.resolve::<PgPort>().0)
        .username(&cfg.resolve::<PgUserName>().0)
        .password(&cfg.resolve::<PgPassword>().0)
        .database(&cfg.resolve::<PgDatabaseName>().0)
        .application_name(&cfg.resolve::<ApplicationName>().0);

    let conn_options = if let Some(level_filter) = cfg.resolve::<SqlxLogLevelFilter>().0 {
        conn_options.log_statements(level_filter)
    } else {
        conn_options
    };

    PgPoolOptions::new()
        .max_connections(cfg.resolve::<SqlxMaxConnections>().0)
        .connect_with(conn_options)
        .await
}
