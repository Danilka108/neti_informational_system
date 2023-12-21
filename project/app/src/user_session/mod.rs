// mod ex;
mod repo;
mod seconds;

use utils::{
    entity::{entity, entity_method, ProvideId},
    outcome::Outcome,
};

use crate::{user, AdaptersModule, AppModule};

// pub use ex::*;
pub use repo::Repo;
pub use seconds::*;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[derive(Debug, Clone)]
#[entity]
pub struct Entity {
    #[id]
    pub id: Id,
    pub refresh_token: String,
    pub expires_at: SecondsFromUnixEpoch,
}

#[derive(Debug, Clone)]
pub struct Id {
    pub user_id: user::EntityId,
    pub metadata: String,
}

#[derive(Debug, Clone, Copy)]
pub struct SessionTTL(pub Seconds);

#[derive(Debug, Clone, Copy)]
pub struct SessionsMaxNumber(pub usize);

// impl Entity {
//     #[entity_method(ctx)]
//     async fn get_validated<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//         metadata: String,
//         refresh_token_to_validate: String,
//     ) -> Outcome<Entity, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         let id = Id::new(CompositeEntityId { metadata, user_id });

//         let session = match repo.find(&id).await.map_repo_ex() {
//             Outcome::Ok(session) => session,
//             Outcome::Ex(Exception::DoesNotExist) => {
//                 let _deleted_sessions = repo.delete_by_user_id(&user_id).await.map_repo_ex()?;
//                 return Outcome::Ex(Exception::DoesNotExist);
//             }
//             Outcome::Ex(ex) => return Outcome::Ex(ex),
//             Outcome::Error(err) => return Outcome::Error(err),
//         };

//         if session.refresh_token != refresh_token_to_validate {
//             return Outcome::Ex(Exception::InvalidRefreshToken);
//         }

//         if session.expires_at.is_expired()? {
//             let _deleted_sessions = repo.delete_by_user_id(&user_id).await.map_repo_ex()?;
//             return Outcome::Ex(Exception::Expired);
//         }

//         Outcome::Ok(session)
//     }

//     #[entity_method(ctx)]
//     pub async fn update<A: AdaptersModule + Clone + Sync>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//         metadata: String,
//         refresh_token_to_validate: String,
//         new_refresh_token: String,
//     ) -> Outcome<Entity, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();

//         let old_session = Self::get_validated(user_id, metadata, refresh_token_to_validate)
//             .exec(ctx.clone())
//             .await?;

//         let new_session = Entity {
//             refresh_token: new_refresh_token,
//             ..old_session
//         };

//         repo.update(new_session).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn save<A: AdaptersModule + Clone + Sync>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//         metadata: String,
//         refresh_token: String,
//     ) -> Outcome<Entity, Exception> {
//         let SessionTTL(ttl) = ctx.adapters.resolve();
//         let SessionsMaxNumber(sessions_max_number) = ctx.adapters.resolve();
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();

//         let expires_at = SecondsFromUnixEpoch::expired_at_from_ttl(ttl)?;

//         let session = Entity {
//             id: Id::new(CompositeEntityId {
//                 user_id,
//                 metadata: metadata.clone(),
//             }),
//             refresh_token,
//             expires_at,
//         };

//         if Self::get(user_id, metadata)
//             .exec(ctx.clone())
//             .await
//             .into_result()?
//             .is_ok()
//         {
//             return repo.update(session).await.map_repo_ex();
//         }

//         let sessions_number = Self::count_not_expired(user_id).exec(ctx.clone()).await?;
//         if sessions_number >= sessions_max_number {
//             return Outcome::Ex(Exception::LimitReached);
//         }

//         repo.insert(session).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn remove<A: AdaptersModule + Clone + Sync>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//         metadata: String,
//         refresh_token_to_validate: String,
//     ) -> Outcome<Entity, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();

//         let session = Self::get_validated(user_id, metadata.clone(), refresh_token_to_validate)
//             .exec(ctx.clone())
//             .await?;

//         repo.delete(&session.id).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//         metadata: String,
//     ) -> Outcome<Entity, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.find(&CompositeEntityId { user_id, metadata }.provide_id())
//             .await
//             .map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn get_all<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//     ) -> Outcome<Vec<Entity>, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.list_by_user_id(&user_id).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     async fn count_not_expired<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         user_id: user::EntityId,
//     ) -> Outcome<usize, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.count_not_expired(&user_id).await.map_repo_ex()
//     }

//     // #[entity_method(ctx)]
//     // pub async fn add_member<A: AdaptersModule>(
//     //     self,
//     //     ctx: AppModule<A>,
//     //     member: Member,
//     // ) -> Outcome<Member, Exception> {
//     //     let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//     //     repo.add_member(&self.id, member).await.map_repo_ex()
//     // }

//     // #[entity_method(ctx)]
//     // pub async fn remove_member<A: AdaptersModule>(
//     //     self,
//     //     ctx: AppModule<A>,
//     //     person_id: impl ProvideId<person::Entity> + Send + Sync,
//     // ) -> Outcome<Member, Exception> {
//     //     let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//     //     repo.remove_member(&self.id, person_id.provide_id())
//     //         .await
//     //         .map_repo_ex()
//     // }

//     // #[entity_method(ctx)]
//     // pub async fn get_member<A: AdaptersModule>(
//     //     self,
//     //     ctx: AppModule<A>,
//     //     person_id: impl ProvideId<person::Entity> + Send + Sync,
//     // ) -> Outcome<Member, Exception> {
//     //     let repo = ctx.adapters.resolve::<BoxedRepo>();
//     //     repo.find_member(&self.id, person_id.provide_id())
//     //         .await
//     //         .map_repo_ex()
//     // }

//     // #[entity_method(ctx)]
//     // pub async fn get_members<A: AdaptersModule>(
//     //     self,
//     //     ctx: AppModule<A>,
//     // ) -> Outcome<Vec<Member>, Exception> {
//     //     let repo = ctx.adapters.resolve::<BoxedRepo>();
//     //     repo.list_members(&self.id).await.map_repo_ex()
//     // }
// }
