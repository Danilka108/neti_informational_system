#![feature(iterator_try_collect)]

mod adapters;
pub mod config;
mod pg;
pub mod transaction;

use std::sync::Arc;

use config::ConfigModule;
use pg::init_pg_conn_pool;
use tokio::sync::Mutex;
use transaction::TransactionModule;

pub struct AdaptersModule<C> {
    config_module: Arc<C>,
    conn: sqlx::PgPool,
}

impl<C: ConfigModule> AdaptersModule<C> {
    pub async fn new(config_module: C) -> Result<Self, anyhow::Error> {
        let conn = init_pg_conn_pool(&config_module).await?;

        Ok(Self {
            config_module: Arc::new(config_module),
            conn,
        })
    }

    pub async fn begin_transaction_scope(&self) -> Result<TransactionModule<C>, anyhow::Error> {
        let txn = Arc::new(Mutex::new(self.conn.begin().await?));
        let txn_module = TransactionModule {
            txn,
            config: Arc::clone(&self.config_module),
        };

        Ok(txn_module)
    }
}
