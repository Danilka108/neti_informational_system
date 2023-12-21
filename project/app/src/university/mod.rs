pub mod ex;
mod repo;

use utils::{
    entity::{entity, entity_method, ProvideId},
    outcome::Outcome,
    repo::BaseRepo,
};

pub use ex::Exception;

use crate::{AdaptersModule, AppModule};

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
#[derive(Clone)]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
}

// pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

// #[async_trait::async_trait]
// pub trait Repo: BaseRepo<Entity> {}

// impl Entity {
//     #[entity_method(ctx)]
//     pub async fn create<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         name: String,
//     ) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();

//         let entity = Entity {
//             id: Default::default(),
//             name,
//         };

//         repo.insert(entity).await.map_repo_ex()
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
