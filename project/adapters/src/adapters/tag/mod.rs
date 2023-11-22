mod model;

use std::sync::Arc;
use tokio::sync::Mutex;

use super::ProvideTxn;
use crate::pg::PgTransaction;
use app::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, UniqualValueError},
    tag::Tag,
    Outcome, SerialId,
};
use model::PgTag;

pub struct PgTagRepository {
    pub txn: Arc<Mutex<PgTransaction>>,
}

impl ProvideTxn for PgTagRepository {
    fn provide_txn(&self) -> Arc<Mutex<PgTransaction>> {
        Arc::clone(&self.txn)
    }
}

#[async_trait::async_trait]
impl app::ports::TagRepository for PgTagRepository {
    async fn insert(&self, tag: Tag<()>) -> Outcome<Tag, EntityAlreadyExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgTag,
            "
                INSERT
                  INTO tags (name)
                  VALUES ($1)
                  ON CONFLICT DO NOTHING
                  RETURNING *;
            ",
            &tag.name
        ))
        .await
    }

    async fn update_name(&self, id: SerialId, name: String) -> Outcome<Tag, UniqualValueError> {
        self.fetch_optional(sqlx::query_as!(
            PgTag,
            "
                UPDATE tags
                  SET
                    name = $1
                  WHERE id = $2
                  RETURNING *;
            ",
            &name,
            id,
        ))
        .await
    }

    async fn delete(&self, id: SerialId) -> Outcome<Tag, EntityDoesNotExistError> {
        self.fetch_optional(sqlx::query_as!(
            PgTag,
            "
                DELETE
                    FROM tags
                    WHERE id = $1
                  RETURNING *;
            ",
            id,
        ))
        .await
    }
}
