use std::sync::Arc;

use anyhow::Context;
use app::{
    person::Person,
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, PersonRepository},
};
use sqlx::FromRow;
use tokio::sync::Mutex;

use crate::pg::PgTransaction;

#[derive(Debug, FromRow)]
struct PgPerson {
    id: i32,
}

impl From<Person> for PgPerson {
    fn from(Person { id }: Person) -> Self {
        Self { id }
    }
}

impl From<PgPerson> for Person {
    fn from(PgPerson { id }: PgPerson) -> Self {
        Self { id }
    }
}

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

        let insert_result = sqlx::query_as!(
            PgPerson,
            "
                INSERT
                    INTO persons
                    DEFAULT VALUES
                    RETURNING id;
            "
        )
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

        let update_result = sqlx::query_as!(
            PgPerson,
            r#"
                UPDATE persons
                    SET id = $1
                    WHERE id = $1
                    RETURNING id;
            "#,
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

        let result = sqlx::query_as!(
            PgPerson,
            r#"
                SELECT id
                    FROM persons
                    WHERE
                        id = $1
            "#,
            id
        )
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
