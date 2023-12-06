use lib_base::{BaseModel, SqlxResultMapperExt};
use lib_common::{outcome::Outcome, result_mapper::IntoResultMapper};

use crate::{PgParams, PgTxn};

const UNIQUE_NAME_CONSTRAINT: &str = "universities_name_key";

#[derive(Clone, Debug, sqlx::FromRow, modql::field::Fields, BaseModel)]
#[sea_query::enum_def]
pub struct Model {
    #[id]
    id: i32,
    name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("university {} not found", id.id)]
pub struct NotFound {
    id: ModelId,
}

#[derive(Debug, thiserror::Error)]
#[error("university name '{name}' is already in use")]
pub struct NameIsAlreadyInUse {
    name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum InsertEx {
    #[error(transparent)]
    NameIsAlreadyInUse(#[from] NameIsAlreadyInUse),
}

pub async fn insert(txn: &mut PgTxn<'_>, model: Model) -> Outcome<Model, InsertEx> {
    let name = model.name.clone();

    lib_base::insert::<PgParams, _>(txn.as_mut(), model)
        .await
        .unwrap()
        .into_result_mapper()
        .on_constraint(UNIQUE_NAME_CONSTRAINT, NameIsAlreadyInUse { name })
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

    lib_base::update::<PgParams, _>(txn.as_mut(), model)
        .await
        .into_result_mapper()
        .on_row_not_found(NotFound { id })
        .on_constraint(UNIQUE_NAME_CONSTRAINT, NameIsAlreadyInUse { name })
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
