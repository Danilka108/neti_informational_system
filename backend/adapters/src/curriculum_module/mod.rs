mod model;

use app::{
    curriculum,
    curriculum_module::{self, Entity, EntityId},
    discipline,
};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fetch_all, fetch_one, fetch_optional, PgTransaction};

use self::model::{CurriculumModules, CurriculumModulesIden};

pub struct PgCurriculumModuleRepo {
    pub txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgCurriculumModuleRepo {
    async fn insert(&self, entity: Entity) -> Result<CurriculumModules, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(CurriculumModulesIden::Table)
            .columns([
                CurriculumModulesIden::DisciplineId,
                CurriculumModulesIden::CurriculumId,
                CurriculumModulesIden::Semester,
            ])
            .values_panic([
                entity.discipline_id.value.into(),
                entity.curriculum_id.value.into(),
                entity.semester.into(),
            ])
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    async fn update(&self, entity: Entity) -> Result<CurriculumModules, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(CurriculumModulesIden::Table)
            .values([
                (
                    CurriculumModulesIden::DisciplineId,
                    entity.discipline_id.value.into(),
                ),
                (
                    CurriculumModulesIden::CurriculumId,
                    entity.curriculum_id.value.into(),
                ),
                (CurriculumModulesIden::Semester, entity.semester.into()),
            ])
            .and_where(Expr::col(CurriculumModulesIden::Id).eq(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, &query).await
    }
}

#[async_trait::async_trait]
impl curriculum_module::Repo for PgCurriculumModuleRepo {
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
            .from_table(CurriculumModulesIden::Table)
            .and_where(Expr::col(CurriculumModulesIden::Id).eq(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(CurriculumModulesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(CurriculumModulesIden::Id).eq(id.value));

        let model = fetch_optional::<CurriculumModules>(&self.txn, &query).await?;
        Ok(model.map(Into::into))
    }

    async fn list_by_discipline_id(
        &self,
        discipline_id: discipline::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(CurriculumModulesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(CurriculumModulesIden::DisciplineId).eq(discipline_id.value));

        let models = fetch_all::<CurriculumModules>(&self.txn, &query).await?;
        let entities = models.into_iter().map(Into::into).collect();

        Ok(entities)
    }

    async fn list_by_curriculum_id(
        &self,
        curriculum_id: curriculum::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut query = Query::select();
        query
            .from(CurriculumModulesIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(CurriculumModulesIden::CurriculumId).eq(curriculum_id.value));

        let models = fetch_all::<CurriculumModules>(&self.txn, &query).await?;
        let entities = models.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}
