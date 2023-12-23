mod model;

use app::{
    attestation::{self, Entity, EntityId},
    curriculum_module, teacher,
};
use sea_query::{Asterisk, Condition, Expr, IntoCondition, JoinType, Query};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::{fetch_all, fetch_one, PgTransaction};

use self::model::{
    AttestationExaminers, AttestationExaminersIden, Attestations, AttestationsIden, JoinRow,
    PgAttestationKind,
};

pub struct PgAttestationRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgAttestationRepo {
    async fn insert(&self, entity: Entity) -> Result<Attestations, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(AttestationsIden::Table)
            .columns([
                AttestationsIden::CurriculumModuleId,
                AttestationsIden::Kind,
                AttestationsIden::Duration,
            ])
            .values_panic([
                entity.curriculum_module_id.value.into(),
                PgAttestationKind::from(entity.kind).to_string().into(),
                entity.duration.0.into(),
            ])
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    async fn update(&self, entity: Entity) -> Result<Attestations, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(AttestationsIden::Table)
            .values([
                (
                    AttestationsIden::CurriculumModuleId,
                    entity.curriculum_module_id.value.into(),
                ),
                (
                    AttestationsIden::Kind,
                    PgAttestationKind::from(entity.kind).to_string().into(),
                ),
                (AttestationsIden::Duration, entity.duration.0.into()),
            ])
            .and_where(Expr::col(AttestationsIden::Id).is(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    // select * from study_groups as sg join study_group_curriculums as c on sg.id = c.study_group_id where sg.x = y;
    async fn select(&self, cond: impl IntoCondition) -> Result<Vec<JoinRow>, anyhow::Error> {
        let attestation_table = AttestationsIden::Table;
        let attestation_id = AttestationsIden::Id;
        let examiner_table = AttestationExaminersIden::Table;
        let examiner_attestation_id = AttestationExaminersIden::AttestationId;

        let on = Expr::col((attestation_table, attestation_id))
            .equals((examiner_table, examiner_attestation_id));

        let mut query = Query::select();
        query
            .from(attestation_table)
            .column(Asterisk)
            .join(JoinType::InnerJoin, examiner_table, on)
            .cond_where(cond);

        fetch_all(&self.txn, &query).await
    }

    fn entity_from_select(select: Vec<JoinRow>) -> Option<Entity> {
        let (models, examiners) = select
            .into_iter()
            .map(|v| (v.attestation, v.examiner))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let Some(model) = models.into_iter().take(1).next() else {
            return None;
        };

        Some(model.into_entity(examiners))
    }

    async fn delete_examiners(&self, id: i32) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(AttestationExaminersIden::Table)
            .and_where(Expr::col(AttestationExaminersIden::AttestationId).is(id));

        fetch_one::<()>(&self.txn, &query).await
    }

    async fn insert_examiners(
        &self,
        id: i32,
        examiners: HashSet<teacher::EntityId>,
    ) -> Result<Vec<AttestationExaminers>, anyhow::Error> {
        let mut models = Vec::new();

        for examiner in examiners {
            let mut query = Query::insert();
            query
                .into_table(AttestationExaminersIden::Table)
                .columns([
                    AttestationExaminersIden::AttestationId,
                    AttestationExaminersIden::ExaminerId,
                ])
                .values_panic([id.into(), examiner.value.into()])
                .returning_all();

            let model = fetch_one::<AttestationExaminers>(&self.txn, &query).await?;
            models.push(model);
        }

        Ok(models)
    }
}

#[async_trait::async_trait]
impl attestation::Repo for PgAttestationRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity.clone()).await?
        } else {
            self.insert(entity.clone()).await?
        };

        self.delete_examiners(model.id).await?;
        let examiners = self.insert_examiners(model.id, entity.examiners).await?;

        Ok(model.into_entity(examiners))
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(AttestationsIden::Table)
            .and_where(Expr::col(AttestationsIden::Id).is(entity.id.value));

        self.delete_examiners(entity.id.value).await?;

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(Expr::col((AttestationsIden::Table, AttestationsIden::Id)).is(id.value))
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn find_by_curriculum_module(
        &self,
        curriculum_module_id: curriculum_module::EntityId,
    ) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(
                Expr::col((
                    AttestationsIden::Table,
                    AttestationsIden::CurriculumModuleId,
                ))
                .is(curriculum_module_id.value),
            )
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn list_by_examiners(
        &self,
        examiners_ids: HashSet<teacher::EntityId>,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let mut cond = Condition::all();

        for examiner_id in examiners_ids {
            let expr = Expr::col((
                AttestationExaminersIden::Table,
                AttestationExaminersIden::ExaminerId,
            ))
            .is(examiner_id.value);
            cond = cond.add(expr);
        }

        let select = self.select(cond).await?;

        let mut groups = HashMap::<i32, Vec<JoinRow>>::new();
        for join_row in select {
            groups
                .entry(join_row.attestation.id)
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
