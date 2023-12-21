mod ex;
mod hasher;
mod repo;

use crate::{user_session, AdaptersModule, AppModule};
use utils::{
    entity::{entity, entity_method, LazyAttr, ProvideId},
    outcome::Outcome,
    repo::{BaseRepo, RepoOutcome},
};

pub use ex::Exception;
pub use hasher::{BoxedPasswordHasher, PasswordHasher};
pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[entity]
#[derive(Debug, Clone)]
pub struct Entity {
    #[id]
    pub id: i32,
    pub email: String,
    pub password: HashedPassword,
    // #[allow(dead_code)]
    // pub sessions: Vec<user_session::EntityId>,
}

#[derive(Debug, Clone)]
pub struct HashedPassword {
    pub value: String,
}

// #[async_trait::async_trait]
// pub trait Repo: BaseRepo<Entity> {
//     async fn find_by_email(&self, email: String) -> RepoOutcome<Entity>;
// }

// impl Entity {
//     pub fn new(id: impl ProvideId<Self>, email: String, password: HashedPassword) -> Self {
//         Self {
//             id: id.provide_id().clone(),
//             email,
//             password,
//             sessions: LazyAttr,
//         }
//     }

//     pub fn id(&self) -> EntityId {
//         self.id.clone()
//     }

//     pub fn email(&self) -> &str {
//         &self.email
//     }

//     pub fn password(&self) -> &HashedPassword {
//         &self.password
//     }

//     #[entity_method(ctx)]
//     pub async fn create<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         email: String,
//         password: String,
//     ) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         let hasher = ctx.adapters.resolve::<BoxedPasswordHasher>();

//         let hashed_password = hasher.hash(password).await?;
//         let entity = Entity {
//             id: Default::default(),
//             email,
//             password: hashed_password,
//             sessions: LazyAttr,
//         };

//         repo.insert(entity).await.map_repo_ex()
//     }

//     #[entity_method(ctx)]
//     pub async fn validate_password<A: AdaptersModule>(
//         &self,
//         ctx: AppModule<A>,
//         pass_to_vailidate: &str,
//     ) -> Outcome<(), Exception> {
//         let hasher = ctx.adapters.resolve::<BoxedPasswordHasher>();

//         if hasher.is_matches(pass_to_vailidate, &self.password).await? {
//             Outcome::Ok(())
//         } else {
//             Outcome::Ex(Exception::InvalidPassword)
//         }
//     }

//     #[entity_method(ctx)]
//     pub async fn update_password<A: AdaptersModule>(
//         mut self,
//         ctx: AppModule<A>,
//         old_pass: &str,
//         new_pass: String,
//     ) -> Outcome<Self, Exception> {
//         let mut repo = ctx.adapters.resolve::<BoxedRepo>();
//         let hasher = ctx.adapters.resolve::<BoxedPasswordHasher>();

//         self.validate_password(old_pass).exec(ctx).await;

//         let new_pass = hasher.hash(new_pass).await?;
//         self.password = new_pass;

//         repo.update(self).await.map_repo_ex()
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
//     pub async fn get_by_email<A: AdaptersModule>(
//         ctx: AppModule<A>,
//         email: String,
//     ) -> Outcome<Self, Exception> {
//         let repo = ctx.adapters.resolve::<BoxedRepo>();
//         repo.find_by_email(email).await.map_repo_ex()
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
