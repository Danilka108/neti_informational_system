mod ex;
mod hasher;

use crate::{AdaptersModule, AppModule};
use utils::{
    entity::{entity, entity_method, ProvideId},
    outcome::Outcome,
    repo::BaseRepo,
};

pub use ex::Exception;
pub use hasher::{BoxedPasswordHasher, PasswordHasher};

#[entity]
#[derive(Clone)]
pub struct Entity {
    #[id]
    pub id: i32,
    pub email: String,
    pub password: HashedPassword,
}

#[derive(Clone)]
pub struct HashedPassword {
    pub value: String,
}

pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[async_trait::async_trait]
pub trait Repo: BaseRepo<Entity> {}

impl Entity {
    #[entity_method(ctx)]
    pub async fn create<A: AdaptersModule>(
        ctx: AppModule<A>,
        email: String,
        password: String,
    ) -> Outcome<Self, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();
        let hasher = ctx.adapters.resolve::<BoxedPasswordHasher>();

        let hashed_password = hasher.hash(password).await?;
        let entity = Entity {
            id: Default::default(),
            email,
            password: hashed_password,
        };

        repo.insert(entity).await.map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn validate_password<A: AdaptersModule>(
        &self,
        ctx: AppModule<A>,
        pass_to_vailidate: &str,
    ) -> Outcome<(), Exception> {
        let hasher = ctx.adapters.resolve::<BoxedPasswordHasher>();

        if hasher.is_matches(pass_to_vailidate, &self.password).await? {
            Outcome::Ok(())
        } else {
            Outcome::Ex(Exception::InvalidPassword)
        }
    }

    #[entity_method(ctx)]
    pub async fn update_password<A: AdaptersModule>(
        mut self,
        ctx: AppModule<A>,
        old_pass: &str,
        new_pass: String,
    ) -> Outcome<Self, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();
        let hasher = ctx.adapters.resolve::<BoxedPasswordHasher>();

        self.validate_password(old_pass).exec(ctx).await;

        let new_pass = hasher.hash(new_pass).await?;
        self.password = new_pass;

        repo.update(self).await.map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn get<A: AdaptersModule>(
        ctx: AppModule<A>,
        id: impl ProvideId<Self> + Send + Sync,
    ) -> Outcome<Self, Exception> {
        let repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.find(id.provide_id()).await.map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn update<A: AdaptersModule>(self, ctx: AppModule<A>) -> Outcome<Self, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.update(self).await.map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn delete<A: AdaptersModule>(self, ctx: AppModule<A>) -> Outcome<Self, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.delete(&self.id).await.map_repo_ex()
    }
}
