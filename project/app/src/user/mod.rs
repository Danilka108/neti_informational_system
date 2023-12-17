mod ex;
mod hasher;
mod seconds;

use crate::{AdaptersModule, AppModule};
use utils::{
    entity::{entity, entity_method, LazyAttr, ProvideId},
    outcome::Outcome,
    repo::{BaseRepo, RepoOutcome},
};

mod sessions;

pub use self::seconds::{Seconds, SecondsFromUnixEpoch};
pub use self::sessions::Sessions;
pub use ex::Exception;
pub use hasher::{BoxedPasswordHasher, PasswordHasher};

#[entity]
#[derive(Debug, Clone)]
pub struct Entity {
    #[id]
    id: i32,
    email: String,
    password: HashedPassword,
    #[allow(dead_code)]
    sessions: LazyAttr,
}

#[derive(Debug, Clone, Copy)]
pub struct SessionTTL(pub Seconds);

#[derive(Debug, Clone, Copy)]
pub struct SessionsMaxNumber(pub usize);

pub struct Session {
    pub metadata: String,
    pub refresh_token: String,
    pub expires_at: SecondsFromUnixEpoch,
}

#[derive(Debug, Clone)]
pub struct HashedPassword {
    pub value: String,
}

pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[async_trait::async_trait]
pub trait Repo: BaseRepo<Entity> {
    async fn find_by_email(&self, email: String) -> RepoOutcome<Entity>;

    async fn add_session(
        &mut self,
        id: &EntityId,
        session: Session,
    ) -> RepoOutcome<Entity, Session>;

    async fn update_session(
        &mut self,
        id: &EntityId,
        session: Session,
    ) -> RepoOutcome<Entity, Session>;

    async fn remove_session(
        &mut self,
        id: &EntityId,
        metadata: String,
    ) -> RepoOutcome<Entity, Session>;

    async fn delete_session(
        &mut self,
        id: &EntityId,
        metadata: String,
    ) -> RepoOutcome<Entity, Session>;

    async fn delete_all_sessions(&mut self, d: &EntityId) -> RepoOutcome<Entity, Vec<Session>>;

    async fn find_session(&self, id: &EntityId, metadata: String) -> RepoOutcome<Entity, Session>;

    async fn list_sessions(&self, id: &EntityId) -> RepoOutcome<Entity, Vec<Session>>;

    async fn count_not_expired_sessions(&self, id: &EntityId) -> RepoOutcome<Entity, usize>;
}

impl Entity {
    pub fn new(id: impl ProvideId<Self>, email: String, password: HashedPassword) -> Self {
        Self {
            id: id.provide_id().clone(),
            email,
            password,
            sessions: LazyAttr,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id.clone()
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &HashedPassword {
        &self.password
    }

    pub fn sessions(&self) -> Sessions<'_> {
        Sessions { entity: self }
    }

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
            sessions: LazyAttr,
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
    pub async fn get_by_email<A: AdaptersModule>(
        ctx: AppModule<A>,
        email: String,
    ) -> Outcome<Self, Exception> {
        let repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.find_by_email(email).await.map_repo_ex()
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
