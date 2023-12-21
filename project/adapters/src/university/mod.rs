mod model;

use app::university::{self, Entity, EntityId};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fetch_one, fetch_optional, PgTransaction};

use self::model::{Universities, UniversitiesIden};

pub struct PgUniversityRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgUniversityRepo {
    async fn insert(&self, entity: Entity) -> Result<Universities, anyhow::Error> {
        let mut query = Query::insert();
        let query = query
            .into_table(UniversitiesIden::Table)
            .columns([UniversitiesIden::Name])
            .values_panic([entity.name.into()])
            .returning_all();

        fetch_one(&self.txn, query).await
    }

    async fn update(&self, entity: Entity) -> Result<Universities, anyhow::Error> {
        let mut query = Query::update();
        let query = query
            .table(UniversitiesIden::Table)
            .values([(UniversitiesIden::Name, entity.name.into())])
            .and_where(Expr::col(UniversitiesIden::Id).is(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, query).await
    }
}

#[async_trait::async_trait]
impl university::Repo for PgUniversityRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity).await?
        } else {
            self.insert(entity).await?
        };

        Ok(model.into())
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        fetch_one::<()>(
            &self.txn,
            Query::delete()
                .from_table(UniversitiesIden::Table)
                .and_where(Expr::col(UniversitiesIden::Id).is(entity.id.value)),
        )
        .await?;

        Ok(())
    }

    async fn find(&mut self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let model = fetch_optional::<Universities>(
            &self.txn,
            Query::select()
                .from(UniversitiesIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(UniversitiesIden::Id).is(id.value)),
        )
        .await?;

        Ok(model.map(Into::into))
    }

    async fn find_by_name(&mut self, name: String) -> Result<Option<Entity>, anyhow::Error> {
        let model = fetch_optional::<Universities>(
            &self.txn,
            Query::select()
                .from(UniversitiesIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(UniversitiesIden::Name).is(name)),
        )
        .await?;

        Ok(model.map(Into::into))
    }
}
