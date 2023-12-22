mod model;

use app::{
    person, subdivision,
    teacher::{self, Entity, EntityId},
};
use sea_query::{Asterisk, Expr, IntoCondition, Query};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::{fetch_all, fetch_one, PgTransaction};

use self::model::{
    JoinRow, PgTeacherKind, TeacherClasses, TeacherClassesIden, Teachers, TeachersIden,
};

pub struct PgTeacherRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgTeacherRepo {
    async fn insert(&self, entity: Entity) -> Result<Teachers, anyhow::Error> {
        let mut query = Query::insert();
        query
            .into_table(TeachersIden::Table)
            .columns([
                TeachersIden::PersonId,
                TeachersIden::Kind,
                TeachersIden::DepartmentId,
            ])
            .values_panic([
                entity.person_id.value.into(),
                PgTeacherKind::from(entity.kind).to_string().into(),
                entity.department_id.value.into(),
            ])
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    async fn update(&self, entity: Entity) -> Result<Teachers, anyhow::Error> {
        let mut query = Query::update();
        query
            .table(TeachersIden::Table)
            .values([
                (TeachersIden::PersonId, entity.person_id.value.into()),
                (
                    TeachersIden::Kind,
                    PgTeacherKind::from(entity.kind).to_string().into(),
                ),
                (
                    TeachersIden::DepartmentId,
                    entity.department_id.value.into(),
                ),
            ])
            .and_where(Expr::col(TeachersIden::Id).is(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, &query).await
    }

    // select * from teachers as t join teacher_classes as c on t.id = c.teacher_id where t.x = y;
    async fn select(&self, cond: impl IntoCondition) -> Result<Vec<JoinRow>, anyhow::Error> {
        let teacher_table = TeachersIden::Table;
        let teacher_id = TeachersIden::Id;
        let class_table = TeacherClassesIden::Table;
        let class_teacher_id = TeacherClassesIden::TeacherId;

        let on = Expr::col((teacher_table, teacher_id)).equals((class_table, class_teacher_id));

        let mut query = Query::select();
        query
            .from(teacher_table)
            .column(Asterisk)
            .join(sea_query::JoinType::InnerJoin, class_table, on)
            .cond_where(cond);

        let results = fetch_all::<JoinRow>(&self.txn, &query).await?;
        Ok(results)
    }

    fn entity_from_select(select: Vec<JoinRow>) -> Option<Entity> {
        let (models, curriculums) = select
            .into_iter()
            .map(|v| (v.teacher, v.class))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let Some(model) = models.into_iter().take(1).next() else {
            return None;
        };

        Some(model.into_entity(curriculums))
    }

    async fn delete_classes(&self, id: i32) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(TeacherClassesIden::Table)
            .and_where(Expr::col(TeacherClassesIden::TeacherId).is(id));

        fetch_one::<()>(&self.txn, &query).await
    }

    async fn insert_classes(
        &self,
        id: i32,
        classes: HashSet<teacher::TeacherClass>,
    ) -> Result<Vec<TeacherClasses>, anyhow::Error> {
        let mut models = Vec::new();

        for class in classes {
            let mut query = Query::insert();
            query
                .into_table(TeacherClassesIden::Table)
                .columns([
                    TeacherClassesIden::TeacherId,
                    TeacherClassesIden::ClassId,
                    TeacherClassesIden::StudyGroupId,
                ])
                .values_panic([
                    id.into(),
                    class.class_id.value.into(),
                    class.study_group_id.value.into(),
                ])
                .returning_all();

            let model = fetch_one::<TeacherClasses>(&self.txn, &query).await?;
            models.push(model);
        }

        Ok(models)
    }
}

#[async_trait::async_trait]
impl teacher::Repo for PgTeacherRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity.clone()).await?
        } else {
            self.insert(entity.clone()).await?
        };

        self.delete_classes(model.id).await?;
        let classes = self.insert_classes(model.id, entity.classes).await?;

        Ok(model.into_entity(classes))
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        let mut query = Query::delete();
        query
            .from_table(TeachersIden::Table)
            .and_where(Expr::col(TeachersIden::Id).is(entity.id.value));

        self.delete_classes(entity.id.value).await?;

        fetch_one::<()>(&self.txn, &query).await?;
        Ok(())
    }

    async fn find(&mut self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(Expr::col((TeachersIden::Table, TeachersIden::Id)).is(id.value))
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn find_by_person_id(
        &mut self,
        person_id: person::EntityId,
    ) -> Result<Option<Entity>, anyhow::Error> {
        let select = self
            .select(Expr::col((TeachersIden::Table, TeachersIden::PersonId)).is(person_id.value))
            .await?;

        let entity = Self::entity_from_select(select);
        Ok(entity)
    }

    async fn list_by_department_id(
        &mut self,
        department_id: subdivision::EntityId,
    ) -> Result<Vec<Entity>, anyhow::Error> {
        let select = self
            .select(
                Expr::col((TeachersIden::Table, TeachersIden::DepartmentId))
                    .is(department_id.value),
            )
            .await?;

        let mut groups = HashMap::<i32, Vec<JoinRow>>::new();
        for join_row in select {
            groups
                .entry(join_row.teacher.id)
                .or_insert(vec![])
                .push(join_row);
        }

        let entities = groups
            .into_iter()
            .map(|(_, v)| Self::entity_from_select(v))
            .filter_map(|v| v)
            .collect();

        Ok(entities)
    }
}
