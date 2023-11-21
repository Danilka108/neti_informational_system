use std::num::NonZeroI32;

use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, UniqualValueError},
    Outcome,
};

use super::Tag;

#[async_trait::async_trait]
pub trait TagRepository {
    async fn insert(&self, tag: Tag<()>) -> Outcome<Tag, EntityAlreadyExistError>;

    async fn update_name(&self, id: NonZeroI32, name: String) -> Outcome<Tag, UniqualValueError>;

    async fn delete(&self, id: NonZeroI32) -> Outcome<Tag, EntityDoesNotExistError>;
}
