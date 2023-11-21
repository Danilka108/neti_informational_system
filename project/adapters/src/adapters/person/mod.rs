use std::{num::NonZeroI32, sync::Arc};
use tokio::sync::Mutex;

use super::ProvideTxn;
use crate::pg::PgTransaction;
use app::{
    person::Person,
    ports::{
        EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError, PersonRepository,
    },
    Outcome,
};

mod model;
use model::PgPerson;

pub struct PgPersonRepository {
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

impl ProvideTxn for PgPersonRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

#[async_trait::async_trait]
impl PersonRepository for PgPersonRepository {
    async fn insert(&self, _person: Person<()>) -> Outcome<Person, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgPerson,
            "INSERT INTO persons DEFAULT VALUES RETURNING id;",
        ))
        .await
    }

    async fn update(&self, person: Person) -> Outcome<Person, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgPerson,
            "UPDATE persons SET id = $1 WHERE id = $1 RETURNING id;",
            person.id.get(),
        ))
        .await
    }

    async fn find_by_id(&self, id: NonZeroI32) -> Outcome<Person, EntityNotFoundError> {
        self.fetch_optional(sqlx::query_as!(
            PgPerson,
            "SELECT id FROM persons WHERE id = $1;",
            id.get()
        ))
        .await
    }
}
