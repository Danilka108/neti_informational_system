use lib_base::{BaseModel, SqlxResultMapperExt};
use lib_common::{outcome::Outcome, result_mapper::IntoResultMapper};
use modql::filter::{OpValInt32, OpValsInt32, OpValsString};

use crate::{PgParams, PgTxn};

const UNIQUE_NAME_PER_UNIVERSITY_CONSTRAINT: &str = "subdivisions_university_id_name_key";

#[derive(Clone, Debug, sqlx::FromRow, modql::field::Fields, BaseModel)]
#[sea_query::enum_def]
pub struct Model {
    #[id]
    pub id: i32,
    pub name: String,
    pub university_id: i32,
}

#[derive(Debug, Default, modql::filter::FilterNodes)]
struct ModelFilter {
    id: Option<OpValsInt32>,
    university_id: Option<OpValsInt32>,
    name: Option<OpValsString>,
}

#[derive(Debug, thiserror::Error)]
#[error("subdivision {} not found", id.id)]
pub struct NotFound {
    id: ModelId,
}

#[derive(Debug, thiserror::Error)]
#[error("subdivision name '{name}' is already in use for university {university_id}")]
pub struct NameIsAlreadyInUse {
    name: String,
    university_id: i32,
}

#[derive(Debug, thiserror::Error)]
pub enum InsertEx {
    #[error(transparent)]
    NameIsAlreadyInUse(#[from] NameIsAlreadyInUse),
}

pub async fn insert(txn: &mut PgTxn<'_>, model: Model) -> Outcome<Model, InsertEx> {
    let name = model.name.clone();
    let university_id = model.university_id;

    lib_base::insert::<PgParams, _>(txn.as_mut(), model)
        .await
        .unwrap()
        .into_result_mapper()
        .on_constraint(
            UNIQUE_NAME_PER_UNIVERSITY_CONSTRAINT,
            NameIsAlreadyInUse {
                name,
                university_id,
            },
        )
        .map()
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateEx {
    #[error(transparent)]
    NotFound(#[from] NotFound),
    #[error(transparent)]
    NameIsAlreadyInUse(#[from] NameIsAlreadyInUse),
}

pub async fn update(txn: &mut PgTxn<'_>, model: Model) -> Outcome<Model, UpdateEx> {
    let id = model.id();
    let name = model.name.clone();
    let university_id = model.university_id;

    lib_base::update::<PgParams, _>(txn.as_mut(), model)
        .await
        .into_result_mapper()
        .on_row_not_found(NotFound { id })
        .on_constraint(
            UNIQUE_NAME_PER_UNIVERSITY_CONSTRAINT,
            NameIsAlreadyInUse {
                name,
                university_id,
            },
        )
        .map()
}

#[derive(Debug, thiserror::Error)]
pub enum FindByIdEx {
    #[error(transparent)]
    NotFound(#[from] NotFound),
}

pub async fn find_by_id(txn: &mut PgTxn<'_>, id: ModelId) -> Outcome<Model, FindByIdEx> {
    lib_base::find_by_id::<PgParams, _>(txn.as_mut(), id.clone())
        .await
        .into_result_mapper()
        .on_row_not_found(NotFound { id })
        .map()
}

#[derive(Debug, thiserror::Error)]
pub enum FindWithUniversityEx {}

pub async fn list_by_univeristy(
    txn: &mut PgTxn<'_>,
    university_id: i32,
    limit: i32,
    offset: i32,
) -> Outcome<Vec<Model>, FindWithUniversityEx> {
    let filter = ModelFilter {
        university_id: Some(OpValsInt32(vec![OpValInt32::Eq(university_id)])),
        ..Default::default()
    };

    lib_base::list::<PgParams, Model, _>(txn.as_mut(), Some(filter), None)
        .await
        .unwrap()
        .into_result_mapper()
        .map()
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteByIdEx {
    #[error(transparent)]
    NotFound(#[from] NotFound),
}

pub async fn delete_by_id(txn: &mut PgTxn<'_>, id: ModelId) -> Outcome<Model, DeleteByIdEx> {
    lib_base::delete_by_id::<PgParams, _>(txn.as_mut(), id.clone())
        .await
        .into_result_mapper()
        .on_row_not_found(NotFound { id })
        .map()
}
