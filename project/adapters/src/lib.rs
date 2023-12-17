#![feature(iterator_try_collect)]

mod person;

type PgTransaction<'c> = sqlx::Transaction<'c, sqlx::Postgres>;

struct PgDbConfig;

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
