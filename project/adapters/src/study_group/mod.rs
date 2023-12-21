use app::study_group::{self, Entity};
use sea_query::{Asterisk, Expr, Query};
use tokio::sync::Mutex;

use self::models::{
    PgQualification, PgTrainingKind, StudyGroupCurriculums, StudyGroupCurriculumsIden, StudyGroups,
    StudyGroupsIden,
};
use crate::{fetch_all, fetch_one, fetch_optional, PgTransaction};

mod models;

pub struct PgStudyGroupRepo {
    txn: std::sync::Arc<Mutex<PgTransaction<'static>>>,
}

#[async_trait::async_trait]
impl study_group::Repo for PgStudyGroupRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model: StudyGroups = if let Some(_) = self.find(entity.id.value).await? {
            fetch_one(
                &self.txn,
                Query::update()
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
                    .and_where(Expr::col(StudyGroupsIden::Id).is(entity.id.value))
                    .returning_all(),
            )
            .await?
        } else {
            fetch_one(
                &self.txn,
                Query::insert()
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
                    .returning_all(),
            )
            .await?
        };

        let _ = delete_curriculums(model.id).await?;

        let mut curriculums = Vec::new();
        for curriculum in entity.curriculums {
            let curriculum: StudyGroupCurriculums = fetch_one(
                &self.txn,
                Query::insert()
                    .into_table(StudyGroupCurriculumsIden::Table)
                    .columns([
                        StudyGroupCurriculumsIden::StudyGroupId,
                        StudyGroupCurriculumsIden::CurriculumId,
                    ])
                    .values_panic([entity.id.value.into(), curriculum.value.into()])
                    .returning_all(),
            )
            .await?;
            curriculums.push(curriculum);
        }

        Ok(model.into_entity(curriculums))
    }

    async fn delete(&mut self, id: i32) -> Result<Entity, anyhow::Error> {
        let model: StudyGroups = fetch_one(
            &self.txn,
            Query::delete()
                .from_table(StudyGroupsIden::Table)
                .and_where(Expr::col(StudyGroupsIden::Id).is(id))
                .returning_all(),
        )
        .await?;

        let curriculums: Vec<StudyGroupCurriculums> = delete_curriculums(model.id).await?;

        Ok(model.into_entity(curriculums))
    }

    async fn find(&mut self, id: i32) -> Result<Option<Entity>, anyhow::Error> {
        let Some(model): Option<StudyGroups> = fetch_optional(
            &self.txn,
            Query::select()
                .from(StudyGroupsIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(StudyGroupsIden::Id).is(id)),
        )
        .await?
        else {
            return Ok(None);
        };

        let curriculums = select_curriculums(model.id).await?;

        Ok(Some(model.into_entity(curriculums)))
    }

    async fn find_by_name(&mut self, name: String) -> Result<Option<Entity>, anyhow::Error> {
        let Some(model): Option<StudyGroups> = fetch_optional(
            &self.txn,
            Query::select()
                .from(StudyGroupsIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(StudyGroupsIden::Name).is(name)),
        )
        .await?
        else {
            return Ok(None);
        };

        let curriculums = select_curriculums(model.id).await?;

        Ok(Some(model.into_entity(curriculums)))
    }
}

async fn select_curriculums(id: i32) -> Result<Vec<StudyGroupCurriculums>, anyhow::Error> {
    fetch_all(
        &self.txn,
        Query::select()
            .from(StudyGroupCurriculumsIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(StudyGroupCurriculumsIden::StudyGroupId).is(id)),
    )
    .await
}

async fn delete_curriculums(id: i32) -> Result<Vec<StudyGroupCurriculums>, anyhow::Error> {
    fetch_all(
        &self.txn,
        Query::delete()
            .from_table(StudyGroupCurriculumsIden::Table)
            .and_where(Expr::col(StudyGroupCurriculumsIden::StudyGroupId).is(id))
            .returning_all(),
    )
    .await?;
}
