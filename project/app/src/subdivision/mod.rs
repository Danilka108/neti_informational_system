mod ex;
mod repo;

use std::collections::HashSet;

use crate::{person, tag, university, AdaptersModule, AppModule};
use utils::{
    entity::{entity, entity_method, LazyAttr, ProvideId},
    outcome::Outcome,
};

// pub use ex::Exception;
pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
#[derive(Debug, Clone)]
pub struct Entity {
    #[id]
    pub id: i32,
    pub name: String,
    pub university_id: university::EntityId,
    pub tags: HashSet<tag::EntityId>,
    pub members: HashSet<Member>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Member {
    pub person_id: person::EntityId,
    pub role: String,
}

// impl Entity {
//     #[entity_method(ctx)]
//     pub async fn create<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         name: String,
//         university_id: university::EntityId,
//     ) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();

//         let entity = Entity {
//             id: Default::default(),
//             tags: Default::default(),
//             members: Default::default(),
//             name,
//             university_id,
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
//     pub async fn list_by_university<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         university_id: impl ProvideId<university::Entity> + Send + Sync,
//     ) -> Outcome<Vec<Entity>, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.list_by_university(university_id.provide_id())
//             .await
//             .map_repo_ex()
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

//     #[entity_method(ctx)]
//     pub async fn add_tag<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//         tag_id: impl ProvideId<tag::Entity> + Send + Sync,
//     ) -> Outcome<tag::Entity, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.add_tag(&self.id, tag_id.provide_id())
//             .await
//             .map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn remove_tag<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//         tag_id: impl ProvideId<tag::Entity> + Send + Sync,
//     ) -> Outcome<tag::Entity, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.remove_tag(&self.id, tag_id.provide_id())
//             .await
//             .map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get_tag<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//         tag_id: impl ProvideId<tag::Entity> + Send + Sync,
//     ) -> Outcome<tag::Entity, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.find_tag(&self.id, tag_id.provide_id())
//             .await
//             .map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get_tags<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//     ) -> Outcome<Vec<tag::Entity>, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.list_tags(&self.id).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn add_member<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//         member: Member,
//     ) -> Outcome<Member, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.add_member(&self.id, member).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn remove_member<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//         person_id: impl ProvideId<person::Entity> + Send + Sync,
//     ) -> Outcome<Member, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.remove_member(&self.id, person_id.provide_id())
//             .await
//             .map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get_member<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//         person_id: impl ProvideId<person::Entity> + Send + Sync,
//     ) -> Outcome<Member, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.find_member(&self.id, person_id.provide_id())
//             .await
//             .map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get_members<A: AdaptersModule>(
//         self,
//         ctx: AppModule<A>,
//     ) -> Outcome<Vec<Member>, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.list_members(&self.id).await.map_repo_ex()
//     }
// }
