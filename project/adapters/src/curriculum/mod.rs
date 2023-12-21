mod model;

use app::curriculum::{self, Entity, EntityId};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{curriculum::model::CurriculumsIden, fetch_one, fetch_optional, PgTransaction};

use self::model::Curriculums;

pub struct PgCurriculumRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgCurriculumRepo {
    async fn insert(&self, entity: Entity) -> Result<Curriculums, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(CurriculumsIden::Table)
            .columns([CurriculumsIden::Name])
            .values_panic([entity.name.into()])
            .returning_all();

        fetch_one::<Curriculums>(&self.txn, &query).await
    }

    async fn update(&self, entity: Entity) -> Result<Curriculums, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(CurriculumsIden::Table)
            .values([(CurriculumsIden::Name, entity.name.into())])
            .and_where(Expr::col(CurriculumsIden::Id).is(entity.id.value))
            .returning_all();

        fetch_one::<Curriculums>(&self.txn, &query).await
    }
}

#[async_trait::async_trait]
impl curriculum::Repo for PgCurriculumRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity).await?
        } else {
            self.insert(entity).await?
        };

        Ok(model.into())
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(CurriculumsIden::Table)
            .and_where(Expr::col(CurriculumsIden::Id).is(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&mut self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(CurriculumsIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(CurriculumsIden::Id).is(id.value));

        let entity = fetch_optional::<Curriculums>(&self.txn, &query)
            .await?
            .map(Into::into);

        Ok(entity)
    }

    async fn find_by_name(&mut self, name: String) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(CurriculumsIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(CurriculumsIden::Name).is(name));

        let entity = fetch_optional::<Curriculums>(&self.txn, &query)
            .await?
            .map(Into::into);

        Ok(entity)
    }
}
