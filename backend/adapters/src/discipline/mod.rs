mod model;

use app::{
    discipline::{self, Entity, EntityId},
    subdivision,
};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fetch_all, fetch_one, fetch_optional, PgTransaction};

use self::model::{Disciplines, DisciplinesIden};

pub struct PgDisciplineRepo {
    pub txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgDisciplineRepo {
    async fn insert(&self, entity: Entity) -> Result<Disciplines, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(DisciplinesIden::Table)
            .columns([DisciplinesIden::Name, DisciplinesIden::DepartmentId])
            .values_panic([entity.name.into(), entity.department_id.value.into()])
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    async fn update(&self, entity: Entity) -> Result<Disciplines, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(DisciplinesIden::Table)
            .values([
                (DisciplinesIden::Name, entity.name.into()),
                (
                    DisciplinesIden::DepartmentId,
                    entity.department_id.value.into(),
                ),
            ])
            .and_where(Expr::col(DisciplinesIden::Id).eq(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, &query).await
    }
}

#[async_trait::async_trait]
impl discipline::Repo for PgDisciplineRepo {
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
            .from_table(DisciplinesIden::Table)
            .and_where(Expr::col(DisciplinesIden::Id).eq(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(DisciplinesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(DisciplinesIden::Id).eq(id.value));

        let model = fetch_optional::<Disciplines>(&self.txn, &query).await?;
        Ok(model.map(Into::into))
    }

    async fn find_by_name(&self, name: String) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(DisciplinesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(DisciplinesIden::Name).eq(name));

        let model = fetch_optional::<Disciplines>(&self.txn, &query).await?;
        Ok(model.map(Into::into))
    }

    async fn list_by_department_id(
        &self,
        department_id: subdivision::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(DisciplinesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(DisciplinesIden::DepartmentId).eq(department_id.value));

        let models = fetch_all::<Disciplines>(&self.txn, &query).await?;
        let entities = models.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}
