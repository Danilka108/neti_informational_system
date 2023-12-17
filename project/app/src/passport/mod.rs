use utils::{
    entity::{entity_method, ProvideId},
    outcome::Outcome,
    repo::{self, BaseRepo},
};

use crate::{user, AdaptersModule, AppModule};

mod exception;
mod number;

pub use exception::Exception;
pub use number::{
    InvalidPassportNumberError, InvalidPassportSeriesError, PassportNumber, PassportSeries,
};

#[utils::entity::entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub person_id: user::EntityId,
    pub first_name: String,
    pub last_name: String,
    pub partonymic: String,
    pub date_of_birth: time::Date,
    pub date_of_issue: time::Date,
    pub number: PassportNumber,
    pub series: PassportSeries,
}

pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[async_trait::async_trait]
pub trait Repo: BaseRepo<Entity> {
    async fn list_by_person(
        &self,
        person_id: &user::EntityId,
    ) -> Outcome<Vec<Entity>, repo::ex::Exception<Entity>>;
}

impl Entity {
    #[entity_method(ctx)]
    pub async fn create<A: AdaptersModule>(self, ctx: AppModule<A>) -> Outcome<Self, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.insert(self).await.map_repo_ex()
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
    pub async fn list_by_person<A: AdaptersModule>(
        ctx: AppModule<A>,
        person_id: impl ProvideId<user::Entity> + Send + Sync,
    ) -> Outcome<Vec<Entity>, Exception> {
        let repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.list_by_person(person_id.provide_id())
            .await
            .map_repo_ex()
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
