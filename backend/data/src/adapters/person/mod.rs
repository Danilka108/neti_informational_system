mod pg_person;

use std::sync::Arc;

use anyhow::Context;
use app::{
    person::Person,
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, PersonRepository},
};
use pg_person::PgPerson;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

pub struct PgPersonRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

#[async_trait::async_trait]
impl PersonRepository for PgPersonRepository {
    async fn insert(
        &self,
        person: Person<()>,
    ) -> Result<Result<Person, EntityAlreadyExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let insert_result =
            sqlx::query_file_as!(PgPerson, "src/adapters/person/scripts/insert.sql",)
                .fetch_optional(conn)
                .await;

        match insert_result {
            Ok(Some(val)) => Ok(Ok(val.into())),
            Ok(None) => Ok(Err(EntityAlreadyExistError)),
            Err(err) => Err(err),
        }
        .context("failed to insert new row into persons table")
    }

    async fn update(
        &self,
        person: Person,
    ) -> Result<Result<Person, EntityDoesNotExistError>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let update_result = sqlx::query_file_as!(
            PgPerson,
            "src/adapters/person/scripts/update.sql",
            person.id
        )
        .fetch_optional(conn)
        .await;

        match update_result {
            Ok(Some(val)) => Ok(Ok(val.into())),
            Ok(None) => Ok(Err(EntityDoesNotExistError)),
            Err(err) => Err(err),
        }
        .context("failed to update row in persons table")
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Person>, anyhow::Error> {
        let conn = &mut **self.txn.lock().await;

        let result =
            sqlx::query_file_as!(PgPerson, "src/adapters/person/scripts/find_by_id.sql", id)
                .fetch_optional(conn)
                .await;

        match result {
            Ok(Some(val)) => Ok(Some(val.into())),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
        .context("failed to select row from persons table")
    }
}
