mod models;

use std::{num::NonZeroI32, sync::Arc};

use app::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    university::University,
    Outcome,
};
use tokio::sync::Mutex;

use super::ProvideTxn;
use crate::pg::PgTransaction;
pub(crate) use models::PgUniversity;

pub struct PgUniveristyRepository {
    pub txn: Arc<Mutex<PgTransaction>>,
}

impl ProvideTxn for PgUniveristyRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

#[async_trait::async_trait]
impl app::ports::UniversityRepository for PgUniveristyRepository {
    async fn insert(
        &self,
        university: University<()>,
    ) -> Outcome<University, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgUniversity,
            "
                INSERT
                    INTO universities (name)
                    VALUES ($1)
                    ON CONFLICT DO NOTHING
                    RETURNING *;
            ",
            university.name
        ))
        .await
    }

    async fn delete(&self, id: NonZeroI32) -> Outcome<University, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgUniversity,
            "
                DELETE
                    FROM universities
                    WHERE id = $1
                    RETURNING *;
            ",
            id.get()
        ))
        .await
    }

    async fn get(&self, id: NonZeroI32) -> Outcome<University, EntityNotFoundError> {
        self.fetch_optional(sqlx::query_as!(
            PgUniversity,
            "
                SELECT id, name
                    FROM universities
                    WHERE id = $1;
            ",
            id.get()
        ))
        .await
    }
}
