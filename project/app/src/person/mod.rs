use utils::{
    entity::{entity_method, ProvideId},
    outcome::Outcome,
    repo::BaseRepo,
};

use crate::{user, AdaptersModule, AppModule};

use self::exception::Exception;

mod exception;
mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[utils::entity::entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub user_id: user::EntityId,
}

// #[async_trait::async_trait]
// pub trait Repo: BaseRepo<Entity> {}

// impl Entity {
//     #[entity_method(ctx)]
//     pub async fn create<A: AdaptersModule>(self, ctx: AppModule<A>) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.insert(self).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         id: impl ProvideId<Self> + Send + Sync,
//     ) -> Outcome<Self, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.find(id.provide_id()).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn update<A: AdaptersModule>(self, ctx: AppModule<A>) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.update(self).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn delete<A: AdaptersModule>(self, ctx: AppModule<A>) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.delete(&self.id).await.map_repo_ex()
//     }
// }
