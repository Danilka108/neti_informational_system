use std::{convert::Infallible, num::NonZeroI32};

use crate::{
    person::Person,
    ports::{
        EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError, UniqualValueError,
    },
    university::University,
    Outcome,
};

use super::{Subdivision, SubdivisionMember, SubdivisionTag};

#[async_trait::async_trait]
pub trait SubdivisionRepository {
    async fn insert(
        &self,
        subdivision: Subdivision<()>,
    ) -> Outcome<Subdivision, EntityAlreadyExistError>;

    async fn delete(&self, id: NonZeroI32) -> Outcome<Subdivision, EntityDoesNotExistError>;

    async fn update_name(
        &self,
        id: NonZeroI32,
        name: String,
    ) -> Outcome<Subdivision, UniqualValueError>;

    async fn get(&self, id: NonZeroI32) -> Outcome<Subdivision, EntityNotFoundError>;

    async fn get_by_university(
        &self,
        university: University,
    ) -> Outcome<Vec<Subdivision>, Infallible>;
}

#[async_trait::async_trait]
pub trait SubdivisionMemberRepository {
    async fn insert(
        &self,
        member: SubdivisionMember,
    ) -> Outcome<SubdivisionMember, EntityAlreadyExistError>;

    async fn remove(
        &self,
        id: (Subdivision, Person),
    ) -> Outcome<SubdivisionMember, EntityDoesNotExistError>;

    async fn get(
        &self,
        id: (Subdivision, Person),
    ) -> Outcome<SubdivisionMember, EntityDoesNotExistError>;

    async fn update_role(
        &self,
        member: SubdivisionMember,
    ) -> Outcome<SubdivisionMember, EntityDoesNotExistError>;

    async fn get_by_subdivison(
        &self,
        subdivision: Subdivision,
    ) -> Outcome<Vec<SubdivisionMember>, Infallible>;
}

#[async_trait::async_trait]
pub trait SubdivisionTagRepository {
    async fn insert(&self, tag: SubdivisionTag)
        -> Outcome<SubdivisionTag, EntityAlreadyExistError>;

    async fn remove(&self, tag: SubdivisionTag)
        -> Outcome<SubdivisionTag, EntityDoesNotExistError>;

    async fn get_by_subdivison(
        &self,
        subdivision: Subdivision,
    ) -> Outcome<Vec<SubdivisionTag>, Infallible>;
}
