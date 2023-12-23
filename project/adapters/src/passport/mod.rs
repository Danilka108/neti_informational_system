mod model;

use app::{
    passport::{self, Entity, EntityId},
    person,
};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    fetch_all, fetch_one, fetch_optional,
    passport::model::{Passports, PgGender},
    PgTransaction,
};

use self::model::PassportsIden;

pub struct PgPassportRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgPassportRepo {
    async fn insert(&self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(PassportsIden::Table)
            .columns([
                PassportsIden::PersonId,
                PassportsIden::FirstName,
                PassportsIden::LastName,
                PassportsIden::Patronymic,
                PassportsIden::DateOfBirth,
                PassportsIden::DateOfIssue,
                PassportsIden::Number,
                PassportsIden::Series,
                PassportsIden::Gender,
            ])
            .values_panic([
                entity.person_id.value.into(),
                entity.first_name.into(),
                entity.last_name.into(),
                entity.patronymic.into(),
                entity.date_of_birth.into(),
                entity.date_of_issue.into(),
                entity.number.to_string().into(),
                entity.series.to_string().into(),
                PgGender::from(entity.gender).to_string().into(),
            ])
            .returning_all();

        let model = fetch_one::<Passports>(&self.txn, &query).await?;
        Ok(model.into())
    }

    async fn update(&self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(PassportsIden::Table)
            .values([
                (PassportsIden::PersonId, entity.person_id.value.into()),
                (PassportsIden::FirstName, entity.first_name.into()),
                (PassportsIden::LastName, entity.last_name.into()),
                (PassportsIden::Patronymic, entity.patronymic.into()),
                (PassportsIden::DateOfBirth, entity.date_of_birth.into()),
                (PassportsIden::DateOfIssue, entity.date_of_issue.into()),
                (PassportsIden::Number, entity.number.to_string().into()),
                (PassportsIden::Series, entity.series.to_string().into()),
                (
                    PassportsIden::Gender,
                    PgGender::from(entity.gender).to_string().into(),
                ),
            ])
            .and_where(Expr::col(PassportsIden::Id).is(entity.id.value))
            .returning_all();

        let model = fetch_one::<Passports>(&self.txn, &query).await?;
        Ok(model.into())
    }
}

#[async_trait::async_trait]
impl passport::Repo for PgPassportRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity).await?
        } else {
            self.insert(entity).await?
        };

        Ok(model)
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(PassportsIden::Table)
            .and_where(Expr::col(PassportsIden::Id).is(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(PassportsIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(PassportsIden::Id).is(id.value));

        let model = fetch_optional::<Passports>(&self.txn, &query).await?;
        Ok(model.map(Into::into))
    }

    async fn list_by_person_id(
        &self,
        person_id: person::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(PassportsIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(PassportsIden::PersonId).is(person_id.value));

        let entities = fetch_all::<Passports>(&self.txn, &query)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(entities)
    }
}
