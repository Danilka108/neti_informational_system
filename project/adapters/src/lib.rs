#![feature(iterator_try_collect)]

use std::sync::Arc;

use sea_query::PostgresQueryBuilder;
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow};
use tokio::sync::Mutex;

mod attestation;
mod class;
mod class_kind;
mod curriculum;
mod curriculum_module;
mod discipline;
mod passport;
mod person;
mod student;
mod study_group;
mod subdivision;
mod tag;
mod teacher;
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
