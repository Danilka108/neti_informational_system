use std::num::NonZeroI32;

use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, UniqualValueError},
    Outcome,
};

use super::{
    exceptions::{CreateTagException, DeleteTagException, UpdateTagNameException},
    BoxedTagRepository, Tag,
};

pub struct TagService {
    pub(crate) repo: BoxedTagRepository,
}

impl TagService {
    pub async fn create(self, name: String) -> Outcome<Tag, CreateTagException> {
        self.repo
            .insert(Tag {
                name: name.clone(),
                id: (),
            })
            .await
            .map_exception(
                |EntityAlreadyExistError| CreateTagException::NameIsAlreadyInUse { name },
            )
    }

    pub async fn update_name(
        self,
        id: NonZeroI32,
        name: String,
    ) -> Outcome<Tag, UpdateTagNameException> {
        self.repo
            .update_name(id, name.clone())
            .await
            .map_exception(|UniqualValueError| UpdateTagNameException::NameIsAlreadyInUse { name })
    }

    pub async fn delete(self, id: NonZeroI32) -> Outcome<Tag, DeleteTagException> {
        self.repo
            .delete(id)
            .await
            .map_exception(|EntityDoesNotExistError| DeleteTagException::TagDoesNotExist { id })
    }
}
