use std::collections::{HashMap, HashSet};

use app::{
    attestation, person,
    student::{self, Entity, EntityId, StudentAttestation},
    study_group,
};
use sea_query::{Asterisk, Condition, Expr, IntoCondition, Query};
use tokio::sync::Mutex;

use self::model::{JoinRow, StudentAttestations, StudentAttestationsIden, Students, StudentsIden};
use crate::{fetch_all, fetch_one, PgTransaction};

mod model;

pub struct PgStudentRepo {
    txn: std::sync::Arc<Mutex<PgTransaction<'static>>>,
}

impl PgStudentRepo {
    async fn insert(&self, entity: Entity) -> Result<Students, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(StudentsIden::Table)
            .columns([StudentsIden::PersonId, StudentsIden::StudyGroupId])
            .values_panic([
                entity.person_id.value.into(),
                entity.study_group_id.value.into(),
            ])
            .returning_all();

        let model = fetch_one(&self.txn, &query).await?;
        Ok(model)
    }

    async fn update(&self, entity: Entity) -> Result<Students, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(StudentsIden::Table)
            .values([
                (StudentsIden::PersonId, entity.person_id.value.into()),
                (
                    StudentsIden::StudyGroupId,
                    entity.study_group_id.value.into(),
                ),
            ])
            .and_where(Expr::col(StudentsIden::Id).is(entity.id.value))
            .returning_all();

        let model = fetch_one(&self.txn, &query).await?;
        Ok(model)
    }

    async fn select(&self, cond: impl IntoCondition) -> Result<Vec<JoinRow>, anyhow::Error> {
        let student_table = StudentsIden::Table;
        let student_id = StudentsIden::Id;
        let attestation_table = StudentAttestationsIden::Table;
        let attestation_student_id = StudentAttestationsIden::StudentId;

        let on = Expr::col((student_table, student_id))
            .equals((attestation_table, attestation_student_id));

        let mut query = Query::select();
        query
            .from(student_table)
            .column(Asterisk)
            .join(sea_query::JoinType::InnerJoin, attestation_table, on)
            .cond_where(cond);

        let results = fetch_all::<JoinRow>(&self.txn, &query).await?;
        Ok(results)
    }

    fn entity_from_select(select: Vec<JoinRow>) -> Option<Entity> {
        let (models, attestations) = select
            .into_iter()
            .map(|v| (v.student, v.attestation))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let Some(model) = models.into_iter().take(1).next() else {
            return None;
        };

        Some(model.into_entity(attestations))
    }

    async fn delete_attestations(&self, id: i32) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(StudentAttestationsIden::Table)
            .and_where(Expr::col(StudentAttestationsIden::StudentId).is(id));

        fetch_one::<()>(&self.txn, &query).await
    }

    async fn insert_attestations(
        &self,
        id: i32,
        attestations: HashSet<StudentAttestation>,
    ) -> Result<Vec<StudentAttestations>, anyhow::Error> {
        let mut models = Vec::new();

        for attestation in attestations {
            let mut query = Query::insert();
            query
                .into_table(StudentAttestationsIden::Table)
                .columns([
                    StudentAttestationsIden::StudentId,
                    StudentAttestationsIden::AttestationId,
                    StudentAttestationsIden::Score,
                ])
                .values_panic([
                    id.into(),
                    attestation.attestation_id.value.into(),
                    attestation.score.into(),
                ])
                .returning_all();

            let model = fetch_one::<StudentAttestations>(&self.txn, &query).await?;
            models.push(model);
        }

        Ok(models)
    }

    async fn list(&self, cond: impl IntoCondition) -> Result<Vec<Entity>, anyhow::Error> {
        let select = self.select(cond).await?;

        let mut groups = HashMap::<i32, Vec<JoinRow>>::new();
        for join_row in select {
            groups
                .entry(join_row.student.id)
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

#[async_trait::async_trait]
impl student::Repo for PgStudentRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity.clone()).await?
        } else {
            self.insert(entity.clone()).await?
        };

        self.delete_attestations(model.id).await?;
        let attestations = self
            .insert_attestations(model.id, entity.attestations)
            .await?;

        Ok(model.into_entity(attestations))
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(StudentsIden::Table)
            .and_where(Expr::col(StudentsIden::Id).is(entity.id.value));

        fetch_one::<()>(&self.txn, &query).await?;
        let _ = self.delete_attestations(entity.id.value).await?;

        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(Expr::col((StudentsIden::Table, StudentsIden::Id)).is(id.value))
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn list_by_person(
        &self,
        person_id: person::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        self.list(Expr::col((StudentsIden::Table, StudentsIden::PersonId)).eq(person_id.value))
            .await
    }

    async fn list_by_study_group(
        &self,
        study_group_id: study_group::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        self.list(
            Expr::col((StudentsIden::Table, StudentsIden::StudyGroupId)).eq(study_group_id.value),
        )
        .await
    }

    async fn list_by_attestations(
        &self,
        attestations_ids: HashSet<attestation::EntityId>,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut cond = Condition::all();

        for attestation_id in attestations_ids {
            let expr = Expr::col((
                StudentAttestationsIden::Table,
                StudentAttestationsIden::AttestationId,
            ))
            .is(attestation_id.value);

            cond = cond.add(expr);
        }

        self.list(cond).await
    }
}
