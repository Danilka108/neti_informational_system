use crate::{
    person::Person,
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, UniqualValueError},
    university::University,
};

use super::{Subdivision, SubdivisionId, SubdivisionMember, SubdivisionTag};

#[async_trait::async_trait]
pub trait SubdivisionRepository {
    async fn insert(
        &self,
        subdivision: Subdivision,
    ) -> Result<Result<Subdivision, EntityAlreadyExistError>, anyhow::Error>;

    async fn delete(
        &self,
        subdivision: SubdivisionId,
    ) -> Result<Result<Subdivision, EntityDoesNotExistError>, anyhow::Error>;

    async fn update_name(
        &self,
        subdivision: Subdivision,
    ) -> Result<Result<Subdivision, UniqualValueError>, anyhow::Error>;

    async fn get(&self, id: SubdivisionId) -> Result<Subdivision, anyhow::Error>;

    async fn get_by_university(
        &self,
        university: University,
    ) -> Result<Vec<Subdivision>, anyhow::Error>;
}

#[async_trait::async_trait]
pub trait SubdivisionMemberRepository {
    async fn insert(
        &self,
        member: SubdivisionMember,
    ) -> Result<Result<SubdivisionMember, EntityAlreadyExistError>, anyhow::Error>;

    async fn remove(
        &self,
        member: (Subdivision, Person),
    ) -> Result<Result<SubdivisionMember, EntityDoesNotExistError>, anyhow::Error>;

    async fn get(
        &self,
        id: (Subdivision, Person),
    ) -> Result<Result<SubdivisionMember, EntityDoesNotExistError>, anyhow::Error>;

    async fn get_by_subdivison(
        &self,
        subdivsion: Subdivision,
    ) -> Result<Vec<SubdivisionMember>, anyhow::Error>;
}

#[async_trait::async_trait]
pub trait SubdivisionTagRepository {
    async fn insert(
        &self,
        member: SubdivisionTag,
    ) -> Result<Result<SubdivisionTag, EntityAlreadyExistError>, anyhow::Error>;

    async fn remove(
        &self,
        member: SubdivisionTag,
    ) -> Result<Result<SubdivisionTag, EntityDoesNotExistError>, anyhow::Error>;

    async fn get_by_subdivison(
        &self,
        subdivsion: Subdivision,
    ) -> Result<Vec<SubdivisionTag>, anyhow::Error>;
}
