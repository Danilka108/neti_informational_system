#![feature(iterator_try_collect)]

use std::sync::Arc;

use sea_query::PostgresQueryBuilder;
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow};
use tokio::sync::Mutex;

mod curriculum;
mod passport;
mod person;
mod subdivision;
mod tag;
mod university;
mod user;
mod user_session;

type PgTransaction<'c> = sqlx::Transaction<'c, sqlx::Postgres>;

async fn fetch_one<M: for<'r> FromRow<'r, PgRow> + Send + Unpin>(
    txn: &Arc<Mutex<PgTransaction<'static>>>,
    query: &(impl SqlxBinder + Send),
) -> Result<M, anyhow::Error> {
    let (sql, args) = query.build_sqlx(PostgresQueryBuilder);
    let model = sqlx::query_as_with(&sql, args)
        .fetch_one(txn.lock().await.as_mut())
        .await?;

    Ok(model)
}

async fn fetch_all<M: for<'r> FromRow<'r, PgRow> + Send + Unpin>(
    txn: &Arc<Mutex<PgTransaction<'static>>>,
    query: &(impl SqlxBinder + Send),
) -> Result<Vec<M>, anyhow::Error> {
    let (sql, args) = query.build_sqlx(PostgresQueryBuilder);
    let models = sqlx::query_as_with(&sql, args)
        .fetch_all(txn.lock().await.as_mut())
        .await?;

    Ok(models)
}

async fn fetch_optional<M: for<'r> FromRow<'r, PgRow> + Send + Unpin>(
    txn: &Arc<Mutex<PgTransaction<'static>>>,
    query: &(impl SqlxBinder + Send),
) -> Result<Option<M>, anyhow::Error> {
    let (sql, args) = query.build_sqlx(PostgresQueryBuilder);
    let models = sqlx::query_as_with(&sql, args)
        .fetch_optional(txn.lock().await.as_mut())
        .await?;

    Ok(models)
}

// impl DbConfig for PgDbConfig {
//     type DbDriver = sqlx::Postgres;
//     type QueryBuilder = sea_query::PostgresQueryBuilder;
//     type Executor<'c> = &'c mut sqlx::PgConnection;
// }

// mod adapters;
// pub mod config;
// mod pg;
// pub mod transaction;

// use std::sync::Arc;

// use config::ConfigModule;
// use pg::init_pg_conn_pool;
// use tokio::sync::Mutex;
// use transaction::TransactionModule;

// #[derive(Debug, Clone)]
// pub struct AdaptersModule<C> {
//     config_module: C,
//     conn: sqlx::PgPool,
// }

// impl<C: ConfigModule + Clone> AdaptersModule<C> {
//     pub async fn new(config_module: C) -> Result<Self, anyhow::Error> {
//         let conn = init_pg_conn_pool(&config_module).await?;

//         Ok(Self {
//             config_module: config_module.clone(),
//             conn,
//         })
//     }

//     pub async fn begin_transaction_scope(&self) -> Result<TransactionModule<C>, anyhow::Error> {
//         let txn = Arc::new(Mutex::new(self.conn.begin().await?));
//         let txn_module = TransactionModule {
//             txn,
//             config: self.config_module.clone(),
//         };

//         Ok(txn_module)
//     }
// }
