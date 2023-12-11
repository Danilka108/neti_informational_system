use super::{Entity, EntityId, Member};
use crate::{person, tag, university};
use utils::repo::RepoOutcome;

pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[async_trait::async_trait]
pub trait Repo: utils::repo::BaseRepo<Entity> {
    async fn list_by_university(
        &self,
        university_id: &university::EntityId,
    ) -> RepoOutcome<Entity, Vec<Entity>>;

    async fn add_tag(
        &mut self,
        id: &EntityId,
        tag_id: &tag::EntityId,
    ) -> RepoOutcome<Entity, tag::Entity>;

    async fn remove_tag(
        &mut self,
        id: &EntityId,
        tag_id: &tag::EntityId,
    ) -> RepoOutcome<Entity, tag::Entity>;

    async fn delete_tag(
        &mut self,
        id: &EntityId,
        tag_id: &tag::EntityId,
    ) -> RepoOutcome<Entity, tag::Entity>;

    async fn find_tag(
        &self,
        id: &EntityId,
        tag_id: &tag::EntityId,
    ) -> RepoOutcome<Entity, tag::Entity>;

    async fn list_tags(&self, id: &EntityId) -> RepoOutcome<Entity, Vec<tag::Entity>>;

    async fn add_member(&mut self, id: &EntityId, member: Member) -> RepoOutcome<Entity, Member>;

    async fn remove_member(
        &mut self,
        id: &EntityId,
        person_id: &person::EntityId,
    ) -> RepoOutcome<Entity, Member>;

    async fn delete_member(
        &mut self,
        id: &EntityId,
        person_id: &person::EntityId,
    ) -> RepoOutcome<Entity, Member>;

    async fn find_member(
        &self,
        id: &EntityId,
        person_id: &person::EntityId,
    ) -> RepoOutcome<Entity, Member>;

    async fn list_members(&self, id: &EntityId) -> RepoOutcome<Entity, Vec<Member>>;
}
