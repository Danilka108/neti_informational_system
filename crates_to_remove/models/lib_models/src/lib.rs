// #[derive(Debug, Clone, sqlx::FromRow, modql::field::Fields, lib_base::BaseModel)]

macro_rules! model {
    (pk_columns {$($pk_col_name:ident : $pk_col_ty:ty $pk_col_filter_ty:ty),*$(,)?}; columns {$($col_name:ident : $col_ty:ty filtered_by $col_filter_ty:ty),*$(,)?};) => {
        #[derive(Debug, Clone, sqlx::FromRow, modql::field::Fields)]
        #[sea_query::enum_def]
        pub struct Model {
            $(pub $pk_col_name:$pk_col_ty,)*
            $(pub $col_name:$col_ty,)*
        }

        #[derive(Debug, Clone, modql::field::Fields)]
        pub struct ModelId {
            $(pub $pk_col_name:$pk_col_ty,)*
        }

        #[derive(Debug, Default, modql::filter::FilterNodes)]
        pub struct ModelFilter {
            $(pub $pk_col_name:$pk_col_filter_ty,)*
            $(pub $col_name:$col_filter_ty,)*
        }
    };
}

// model! {
//     pk_columns {
//         id: (i32, sdf),
//     };
//     columns {
//         filter() name: String
//     };
// }

// model!(
//     MyModel {
//         pub(crate) id: i32,
//     }
// );

use sqlx::{PgConnection, Postgres};

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

pub trait BaseModel:
    Clone + modql::field::HasFields + Send + Unpin + for<'r> sqlx::FromRow<'r>
{
    type Id: Clone + modql::field::HasFields;

    fn id(&self) -> Self::Id;
    fn table_ref() -> sea_query::TableRef;
}

// trait Model<'c, P>
// where
//     Self: Send + Unpin + BaseModel + for<'r> FromRow<'r, <P::Database as sqlx::Database>::Row>,
//     P: DbParams<'c>,
// {
// }

// impl<'c, P, M> Model<'c, P> for M
// where
//     P: DbParams<'c>,
//     M: Send + Unpin + BaseModel + for<'r> FromRow<'r, <P::Database as sqlx::Database>::Row>,
// {
// }

// pub mod person;
// pub mod subdivision;
// pub mod university;

// pub type PgTxn<'c> = sqlx::Transaction<'c, Postgres>;

// pub struct PgParams;

// impl<'c> lib_base::DbParams<'c> for PgParams {
//     type Executor = &'c mut PgConnection;
//     type Database = sqlx::Postgres;
//     type QueryBuilder = sea_query::PostgresQueryBuilder;
// }
