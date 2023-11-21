use std::{convert::Infallible, sync::Arc};

use anyhow::{anyhow, Context};
use app::Outcome;
use sqlx::{
    postgres::{PgArguments, PgRow},
    Execute, Postgres,
};
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

pub mod password;
pub mod person;
pub mod session;
pub mod subdivision;
pub mod tag;
pub mod tokens;
pub mod university;
pub mod user;

trait IntoEntity<E> {
    fn into_entity(self) -> Result<E, anyhow::Error>;
}

#[async_trait::async_trait]
trait ProvideTxn {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>>;

    async fn fetch_optional<'q, Model, Entity, NothingError, F>(
        &self,
        query: sqlx::query::Map<'q, Postgres, F, PgArguments>,
    ) -> Outcome<Entity, NothingError>
    where
        NothingError: Default,
        Model: Send + Unpin + IntoEntity<Entity>,
        F: FnMut(PgRow) -> Result<Model, sqlx::Error> + Send,
    {
        let txn = self.provide_txn();
        let conn = &mut **txn.lock().await;

        let err_msg = format!("failed to fetch query {}", query.sql());
        let query_result = query.fetch_optional(conn).await.context(err_msg);

        match query_result {
            Ok(Some(model)) => match model.into_entity() {
                Ok(entity) => Outcome::Success(entity),
                Err(err) => Outcome::Unexpected(err),
            },
            Ok(None) => Outcome::Exception(NothingError::default()),
            Err(err) => Outcome::Unexpected(err),
        }
    }

    async fn fetch_one<'q, Model, Entity, F>(
        &self,
        query: sqlx::query::Map<'q, Postgres, F, PgArguments>,
    ) -> Outcome<Entity, Infallible>
    where
        Model: Send + Unpin + IntoEntity<Entity>,
        F: FnMut(PgRow) -> Result<Model, sqlx::Error> + Send,
    {
        let txn = self.provide_txn();
        let conn = &mut **txn.lock().await;

        let sql = query.sql().to_owned();
        let query_result = query
            .fetch_optional(conn)
            .await
            .context(format!("failed to fetch query {}", sql.clone()));

        match query_result {
            Ok(Some(model)) => match model.into_entity() {
                Ok(entity) => Outcome::Success(entity),
                Err(err) => Outcome::Unexpected(err),
            },
            Ok(None) => Outcome::Unexpected(anyhow!(
                "failed to fetch query {}, expected one value but got nothing",
                sql
            )),
            Err(err) => Outcome::Unexpected(err),
        }
    }

    async fn fetch_all<'q, Model, Entity, F>(
        &self,
        query: sqlx::query::Map<'q, Postgres, F, PgArguments>,
    ) -> Outcome<Vec<Entity>, Infallible>
    where
        Model: Send + Unpin + IntoEntity<Entity>,
        F: FnMut(PgRow) -> Result<Model, sqlx::Error> + Send,
    {
        let txn = self.provide_txn();
        let conn = &mut **txn.lock().await;

        let sql = query.sql().to_owned();
        let query_result = query
            .fetch_all(conn)
            .await
            .context(format!("failed to fetch query {}", sql.clone()));

        match query_result {
            Ok(models) => match models.into_iter().map(|v| v.into_entity()).try_collect() {
                Ok(entities) => Outcome::Success(entities),
                Err(err) => Outcome::Unexpected(err),
            },
            Err(err) => Outcome::Unexpected(err),
        }
    }
}
