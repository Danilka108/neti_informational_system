#![feature(iterator_try_collect)]

use std::sync::Arc;

use sea_query::PostgresQueryBuilder;
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow, PgPool};
use tokio::sync::Mutex;

mod access_token;
mod attestation;
mod class;
mod class_kind;
mod curriculum;
mod curriculum_module;
mod discipline;
mod hasher;
mod passport;
mod person;
mod refresh_token;
mod student;
mod study_group;
mod subdivision;
mod tag;
mod teacher;
mod university;
mod user;
mod user_session;

pub mod config;
mod pg;
mod transaction_module;

use config::ConfigModule;
use pg::init_pg_conn_pool;
pub use transaction_module::TransactionModule;

type PgTransaction<'c> = sqlx::Transaction<'c, sqlx::Postgres>;

#[derive(Debug, Clone)]
pub struct AdaptersModule<C> {
    pub config: C,
    pub(crate) conn: PgPool,
}

impl<C: ConfigModule> AdaptersModule<C> {
    pub async fn new(config: C) -> Result<Self, anyhow::Error> {
        let conn = init_pg_conn_pool(&config).await?;

        Ok(Self { config, conn })
    }

    pub async fn begin_transaction_scope(&self) -> Result<TransactionModule<C>, anyhow::Error> {
        let txn = Arc::new(Mutex::new(self.conn.begin().await?));
        let txn_module = TransactionModule {
            txn,
            config: self.config.clone(),
        };

        Ok(txn_module)
    }
}

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
