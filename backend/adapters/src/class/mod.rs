mod model;

use app::{
    class::{self, Entity, EntityId},
    curriculum_module,
};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fetch_all, fetch_one, fetch_optional, PgTransaction};

use self::model::{Classes, ClassesIden};

pub struct PgClassRepo {
    pub txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgClassRepo {
    async fn insert(&self, entity: Entity) -> Result<Classes, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(ClassesIden::Table)
            .columns([ClassesIden::CurriculumModuleId, ClassesIden::KindName])
            .values_panic([
                entity.curriculum_module_id.value.into(),
                entity.kind_name.value.into(),
            ])
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    async fn update(&self, entity: Entity) -> Result<Classes, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(ClassesIden::Table)
            .values([
                (
                    ClassesIden::CurriculumModuleId,
                    entity.curriculum_module_id.value.into(),
                ),
                (ClassesIden::KindName, entity.kind_name.value.into()),
            ])
            .and_where(Expr::col(ClassesIden::Id).eq(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, &query).await
    }
}

#[async_trait::async_trait]
impl class::Repo for PgClassRepo {
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
            .from_table(ClassesIden::Table)
            .and_where(Expr::col(ClassesIden::Id).eq(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(ClassesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(ClassesIden::Id).eq(id.value));

        let model = fetch_optional::<Classes>(&self.txn, &query).await?;
        Ok(model.map(Into::into))
    }

    async fn list_by_curriculum_module(
        &self,
        curriculum_module_id: curriculum_module::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(ClassesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(ClassesIden::CurriculumModuleId).eq(curriculum_module_id.value));

        let models = fetch_all::<Classes>(&self.txn, &query).await?;
        let entities = models.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}
