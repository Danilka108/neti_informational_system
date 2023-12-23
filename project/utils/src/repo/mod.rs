use crate::{
    entity::{EntityTrait, Id},
    outcome::Outcome,
};

pub mod case;
pub mod ex;
// pub mod mapper;

// #[cfg_attr(feature = "sqlx", path = "sqlx.rs")] pub mod sqlx;

pub type RepoOutcome<Entity, Ok = Entity> = Outcome<Ok, ex::Exception<Entity>>;

#[async_trait::async_trait]
pub trait BaseRepo<Entity: EntityTrait> {
    async fn insert(&mut self, entity: Entity) -> RepoOutcome<Entity>;

    async fn update(&mut self, entity: Entity) -> RepoOutcome<Entity>;

    async fn delete(&mut self, id: &Id<Entity>) -> RepoOutcome<Entity>;

    async fn find(&self, id: &Id<Entity>) -> RepoOutcome<Entity>;
}
