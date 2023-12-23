use crate::{fetch_one, fetch_optional, person::models::PersonsIden, PgTransaction};

mod models;

use app::{
    person::{self, Entity, EntityId},
    user,
};
use sea_query::{Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use self::models::Persons;

pub struct PgPersonRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgPersonRepo {
    async fn insert(&self, entity: Entity) -> Result<Persons, anyhow::Error> {
        let mut query = Query::insert();
        let query = query
            .into_table(PersonsIden::Table)
            .columns([PersonsIden::UserId, PersonsIden::FullName])
            .values_panic([entity.user_id.value.into(), entity.full_name.into()])
            .returning_all();

        fetch_one(&self.txn, query).await
    }

    async fn update(&self, entity: Entity) -> Result<Persons, anyhow::Error> {
        let mut query = Query::update();
        let query = query
            .table(PersonsIden::Table)
            .values([
                (PersonsIden::UserId, entity.user_id.value.into()),
                (PersonsIden::FullName, entity.full_name.into()),
            ])
            .and_where(Expr::col(PersonsIden::Id).is(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, query).await
    }
}

#[async_trait::async_trait]
impl person::Repo for PgPersonRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        if self.find(entity.id).await?.is_some() {
            self.update(entity).await
        } else {
            self.insert(entity).await
        }
        .map(Into::into)
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        let query = query
            .from_table(PersonsIden::Table)
            .and_where(Expr::col(PersonsIden::Id).is(entity.id.value));

        fetch_one::<()>(&self.txn, query).await
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        let query = query
            .from(PersonsIden::Table)
            .and_where(Expr::col(PersonsIden::Id).is(id.value));

        let model = fetch_optional::<Persons>(&self.txn, query).await?;
        Ok(model.map(Into::into))
    }

    async fn find_by_user_id(
        &self,
        user_id: user::EntityId,
    ) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        let query = query
            .from(PersonsIden::Table)
            .and_where(Expr::col(PersonsIden::UserId).is(user_id.value));

        let model = fetch_optional::<Persons>(&self.txn, query).await?;
        Ok(model.map(Into::into))
    }
}
