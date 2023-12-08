use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, UniqualValueError},
    Outcome, SerialId,
};

use super::Tag;

#[async_trait::async_trait]
pub trait TagRepository {
    async fn insert(&self, tag: Tag<()>) -> Outcome<Tag, EntityAlreadyExistError>;

    async fn update_name(&self, id: SerialId, name: String) -> Outcome<Tag, UniqualValueError>;

    async fn delete(&self, id: SerialId) -> Outcome<Tag, EntityDoesNotExistError>;
}
