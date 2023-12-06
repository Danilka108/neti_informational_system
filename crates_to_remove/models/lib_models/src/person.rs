use lib_base::{BaseModel, SqlxResultMapperExt};
use lib_common::{outcome::Outcome, result_mapper::IntoResultMapper};

use crate::{PgParams, PgTxn};

#[derive(Debug, Clone, sqlx::FromRow, modql::field::Fields, lib_base::BaseModel)]
#[sea_query::enum_def]
pub struct Model {
    #[id]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("person {} not found", id.id)]
pub struct NotFound {
    id: ModelId,
}

#[derive(Debug, thiserror::Error)]
pub enum InsertEx {}

pub async fn insert(txn: &mut PgTxn<'_>, model: Model) -> Outcome<Model, InsertEx> {
    lib_base::insert::<PgParams, _>(txn.as_mut(), model)
        .await
        .unwrap()
        .into_result_mapper()
        .map()
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateEx {
    #[error(transparent)]
    NotFound(#[from] NotFound),
}

pub async fn update(txn: &mut PgTxn<'_>, model: Model) -> Outcome<Model, UpdateEx> {
    let id = model.id();

    lib_base::update::<PgParams, _>(txn.as_mut(), model)
        .await
        .into_result_mapper()
        .on_row_not_found(NotFound { id })
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
