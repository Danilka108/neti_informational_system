use std::collections::{HashMap, HashSet};

use app::{
    curriculum,
    study_group::{self, Entity, EntityId},
};
use sea_query::{Asterisk, Condition, ConditionalStatement, Expr, IntoCondition, Query, Value};
use tokio::sync::Mutex;

use self::models::{
    JoinRow, PgQualification, PgTrainingKind, StudyGroupCurriculums, StudyGroupCurriculumsIden,
    StudyGroups, StudyGroupsIden,
};
use crate::{fetch_all, fetch_one, PgTransaction};

mod models;

pub struct PgStudyGroupRepo {
    pub txn: std::sync::Arc<Mutex<PgTransaction<'static>>>,
}

impl PgStudyGroupRepo {
    async fn insert(&self, entity: Entity) -> Result<StudyGroups, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(StudyGroupsIden::Table)
            .columns([
                StudyGroupsIden::Name,
                StudyGroupsIden::StudyingQualification,
                StudyGroupsIden::TrainingKind,
                StudyGroupsIden::DepartmentId,
            ])
            .values_panic([
                entity.name.into(),
                PgQualification::from(entity.studying_qualification)
                    .to_string()
                    .into(),
                PgTrainingKind::from(entity.training_kind)
                    .to_string()
                    .into(),
                entity.department_id.value.into(),
            ])
            .returning_all();

        let model = fetch_one(&self.txn, &query).await?;
        Ok(model)
    }

    async fn update(&self, entity: Entity) -> Result<StudyGroups, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(StudyGroupsIden::Table)
            .values([
                (StudyGroupsIden::Name, entity.name.into()),
                (
                    StudyGroupsIden::StudyingQualification,
                    PgQualification::from(entity.studying_qualification)
                        .to_string()
                        .into(),
                ),
                (
                    StudyGroupsIden::TrainingKind,
                    PgTrainingKind::from(entity.training_kind)
                        .to_string()
                        .into(),
                ),
                (
                    StudyGroupsIden::DepartmentId,
                    entity.department_id.value.into(),
                ),
            ])
            .and_where(Expr::col(StudyGroupsIden::Id).eq(entity.id.value))
            .returning_all();

        let model = fetch_one(&self.txn, &query).await?;
        Ok(model)
    }

    // select * from study_groups as sg join study_group_curriculums as c on sg.id = c.study_group_id where sg.x = y;
    async fn select(&self, cond: impl IntoCondition) -> Result<Vec<JoinRow>, anyhow::Error> {
        let study_group_table = StudyGroupsIden::Table;
        let curriculum_table = StudyGroupCurriculumsIden::Table;
        let study_group_id = StudyGroupsIden::Id;
        let curriculum_study_group_id = StudyGroupCurriculumsIden::CurriculumId;

        let on = Expr::col((study_group_table, study_group_id))
            .equals((curriculum_table, curriculum_study_group_id));

        let mut query = Query::select();
        query
            .from(study_group_table)
            .column(Asterisk)
            .join(sea_query::JoinType::InnerJoin, curriculum_table, on)
            .cond_where(cond);

        let results = fetch_all::<JoinRow>(&self.txn, &query).await?;
        Ok(results)
    }

    fn entity_from_select(select: Vec<JoinRow>) -> Option<Entity> {
        let (models, curriculums) = select
            .into_iter()
            .map(|v| (v.study_group, v.curriculum))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let Some(model) = models.into_iter().take(1).next() else {
            return None;
        };

        Some(model.into_entity(curriculums))
    }

    async fn delete_curriculums(&self, id: i32) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(StudyGroupCurriculumsIden::Table)
            .and_where(Expr::col(StudyGroupCurriculumsIden::StudyGroupId).eq(id));

        fetch_one::<()>(&self.txn, &query).await
    }

    async fn insert_curriculums(
        &self,
        id: i32,
        curriculums: HashSet<curriculum::EntityId>,
    ) -> Result<Vec<StudyGroupCurriculums>, anyhow::Error> {
        let mut models = Vec::new();

        for curriculum in curriculums {
            let mut query = Query::insert();
            query
                .into_table(StudyGroupCurriculumsIden::Table)
                .columns([
                    StudyGroupCurriculumsIden::StudyGroupId,
                    StudyGroupCurriculumsIden::CurriculumId,
                ])
                .values_panic([id.into(), curriculum.value.into()])
                .returning_all();

            let model = fetch_one::<StudyGroupCurriculums>(&self.txn, &query).await?;
            models.push(model);
        }

        Ok(models)
    }
}

#[async_trait::async_trait]
impl study_group::Repo for PgStudyGroupRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity.clone()).await?
        } else {
            self.insert(entity.clone()).await?
        };

        self.delete_curriculums(model.id).await?;
        let curriculums = self
            .insert_curriculums(model.id, entity.curriculums)
            .await?;

        Ok(model.into_entity(curriculums))
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(StudyGroupsIden::Table)
            .and_where(Expr::col(StudyGroupsIden::Id).eq(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        let _ = self.delete_curriculums(entity.id.value).await?;

        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(Expr::col((StudyGroupsIden::Table, StudyGroupsIden::Id)).eq(id.value))
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn find_by_name(&self, name: String) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(Expr::col((StudyGroupsIden::Table, StudyGroupsIden::Name)).eq(name))
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn list(&self) -> Result<Vec<Entity>, anyhow::Error> {
        let select = self.select(Expr::value(true)).await?;

        let mut groups = HashMap::<i32, Vec<JoinRow>>::new();
        for join_row in select {
            groups
                .entry(join_row.study_group.id)
                .or_insert(vec![])
                .push(join_row);
        }

        let entities = groups
            .into_iter()
            .map(|(_, v)| v)
            .map(Self::entity_from_select)
            .filter_map(|v| v)
            .collect();

        Ok(entities)
    }

    async fn list_by_curriculums(
        &self,
        curriculums_ids: HashSet<curriculum::EntityId>,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut cond = Condition::all();

        for curriculum_id in curriculums_ids {
            let expr = Expr::col((
                StudyGroupCurriculumsIden::Table,
                StudyGroupCurriculumsIden::CurriculumId,
            ))
            .eq(curriculum_id.value);

            cond = cond.add(expr);
        }

        let select = self.select(cond).await?;

        let mut groups = HashMap::<i32, Vec<JoinRow>>::new();
        for join_row in select {
            groups
                .entry(join_row.study_group.id)
                .or_insert(vec![])
                .push(join_row);
        }

        let entities = groups
            .into_iter()
            .map(|(_, v)| v)
            .map(Self::entity_from_select)
            .filter_map(|v| v)
            .collect();

        Ok(entities)
    }
}
