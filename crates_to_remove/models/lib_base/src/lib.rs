mod result_mapper;
pub use result_mapper::SqlxResultMapperExt;

pub use lib_base_derive::BaseModel;

use modql::{
    field::HasFields,
    filter::{FilterGroups, ListOptions},
};
use sea_query::{Condition, Expr, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Executor, FromRow, IntoArguments};

// type ModqlResult<O> = Result<O, modql::Error>;
type SeaQueryResult<O> = Result<O, sea_query::error::Error>;
type SqlxResult<O> = Result<O, sqlx::Error>;
type IntoSeaResult<O> = Result<O, modql::filter::IntoSeaError>;

pub trait BaseModel: Clone + modql::field::HasFields {
    type Id: Clone + modql::field::HasFields;

    fn id(&self) -> Self::Id;
    fn table_ref() -> sea_query::TableRef;
}

pub trait DbParams<'c> {
    type Executor: Executor<'c, Database = Self::Database>;
    type Database: sqlx::Database;
    type QueryBuilder: sea_query::QueryBuilder + Default;
}

trait Model<'c, P>
where
    Self: Send + Unpin + BaseModel + for<'r> FromRow<'r, <P::Database as sqlx::Database>::Row>,
    P: DbParams<'c>,
{
}

impl<'c, P, M> Model<'c, P> for M
where
    P: DbParams<'c>,
    M: Send + Unpin + BaseModel + for<'r> FromRow<'r, <P::Database as sqlx::Database>::Row>,
{
}

pub type Txn<'c, P> = sqlx::Transaction<'c, <P as DbParams<'c>>::Database>;

#[allow(private_bounds)]
pub async fn insert<'c, P, M>(executor: P::Executor, model: M) -> SeaQueryResult<SqlxResult<M>>
where
    P: DbParams<'c>,
    M: Model<'c, P>,
    sea_query_binder::SqlxValues: for<'q> IntoArguments<'q, P::Database>,
{
    let (columns, values) = model.not_none_fields().for_sea_insert();

    let (sql, args) = Query::insert()
        .columns(columns.clone())
        .values(values)?
        .returning(Query::returning().columns(columns))
        .build_sqlx(P::QueryBuilder::default());

    Ok(sqlx::query_as_with(&sql, args).fetch_one(executor).await)
}

#[allow(private_bounds)]
pub async fn update<'c, P, M>(executor: P::Executor, model: M) -> SqlxResult<M>
where
    P: DbParams<'c>,
    M: Model<'c, P>,
    sea_query_binder::SqlxValues: for<'q> IntoArguments<'q, P::Database>,
{
    let id = model.id();
    let values = model.not_none_fields().for_sea_update();

    let (sql, args) = Query::update()
        .table(M::table_ref())
        .values(values)
        .cond_where(where_id_eq::<M>(id))
        .returning_all()
        .build_sqlx(P::QueryBuilder::default());

    sqlx::query_as_with::<P::Database, M, _>(&sql, args)
        .fetch_one(executor)
        .await
}

#[allow(private_bounds)]
pub async fn find_by_id<'c, P, M>(executor: P::Executor, id: M::Id) -> SqlxResult<M>
where
    P: DbParams<'c>,
    M: Model<'c, P>,
    sea_query_binder::SqlxValues: for<'q> IntoArguments<'q, P::Database>,
{
    let columns = M::field_idens();
    let (sql, args) = Query::select()
        .columns(columns)
        .cond_where(where_id_eq::<M>(id))
        .build_sqlx(P::QueryBuilder::default());

    sqlx::query_as_with(&sql, args).fetch_one(executor).await
}

#[allow(private_bounds)]
pub async fn list<'c, P, M, F>(
    executor: P::Executor,
    filters: Option<F>,
    list_options: Option<ListOptions>,
) -> IntoSeaResult<SqlxResult<Vec<M>>>
where
    P: DbParams<'c>,
    M: Model<'c, P>,
    F: Into<FilterGroups>,
    sea_query_binder::SqlxValues: for<'q> IntoArguments<'q, P::Database>,
{
    let columns = M::field_idens();

    let mut query = Query::select();
    query.from(M::table_ref()).columns(columns);

    if let Some(filters) = filters {
        let filters: FilterGroups = filters.into();
        let condition: Condition = filters.try_into()?;
        query.cond_where(condition);
    }

    if let Some(list_options) = list_options {
        list_options.apply_to_sea_query(&mut query);
    }

    let (sql, args) = query.build_sqlx(P::QueryBuilder::default());
    Ok(sqlx::query_as_with(&sql, args).fetch_all(executor).await)
}

#[allow(private_bounds)]
pub async fn delete_by_id<'c, P, M>(executor: P::Executor, id: M::Id) -> SqlxResult<M>
where
    P: DbParams<'c>,
    M: Model<'c, P>,
    sea_query_binder::SqlxValues: for<'q> IntoArguments<'q, P::Database>,
{
    let columns = M::field_idens();
    let (sql, args) = Query::delete()
        .cond_where(where_id_eq::<M>(id))
        .returning(Query::returning().columns(columns))
        .build_sqlx(P::QueryBuilder::default());

    sqlx::query_as_with(&sql, args).fetch_one(executor).await
}

fn where_id_eq<M: BaseModel>(id: M::Id) -> Condition {
    let mut cond = Condition::all();

    for field in id.all_fields() {
        cond = cond.add(Expr::col(field.column_ref).is(field.value));
    }

    cond
}
