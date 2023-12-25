use std::marker::PhantomData;

use crate::{entity::EntityTrait, outcome::Outcome};

use super::ex::Exception;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct SqlxMapper<Entity: EntityTrait, Ok> {
    result: Result<Ok, Either<sqlx::Error, Exception<Entity>>>,
}

pub trait IntoSqlxMapper<Entity: EntityTrait, Ok> {
    fn into_sqlx_mapper(self) -> SqlxMapper<Entity, Ok>;
}

impl<Entity: EntityTrait, Ok> IntoSqlxMapper<Entity, Ok> for Result<Ok, sqlx::Error> {
    fn into_sqlx_mapper(self) -> SqlxMapper<Entity, Ok> {
        SqlxMapper {
            result: self.map_err(Either::Left),
        }
    }
}

impl<Entity: EntityTrait, Ok> SqlxMapper<Entity, Ok> {
    pub fn case<Attrs: IntoIterator<Item = Entity::Attr>>(
        mut self,
        case: SqlxCase<Entity, Attrs>,
    ) -> Self {
        self.result = match self.result {
            Err(err) => match err {
                Either::Left(err) => Err(case.map(err)),
                v => Err(v),
            },
            v => v,
        };

        self
    }

    pub fn map(self) -> Outcome<Ok, Exception<Entity>> {
        match self.result {
            Ok(ok) => Outcome::Ok(ok),
            Err(Either::Left(err)) => Outcome::Error(anyhow::Error::new(err)),
            Err(Either::Right(ex)) => Outcome::Ex(ex),
        }
    }
}

enum Kind {
    UniqueConstraint,
    ForeignKeyConstraint,
    CheckConstraint,
    NotFound,
}

pub struct Nothing;

pub struct SqlxCase<Entity, Attrs> {
    kind: Kind,
    constraint: &'static str,
    attrs: Attrs,
    _entity: PhantomData<Entity>,
}

impl SqlxCase<Nothing, Nothing> {
    pub const fn unique_constraint<Entity>(constraint: &'static str) -> SqlxCase<Entity, Nothing> {
        SqlxCase {
            kind: Kind::UniqueConstraint,
            constraint,
            attrs: Nothing,
            _entity: PhantomData,
        }
    }

    pub const fn foreign_key_constraint<Entity>(
        constraint: &'static str,
    ) -> SqlxCase<Entity, Nothing> {
        SqlxCase {
            kind: Kind::ForeignKeyConstraint,
            constraint,
            attrs: Nothing,
            _entity: PhantomData,
        }
    }

    pub const fn check_constraint<Entity>(constraint: &'static str) -> SqlxCase<Entity, Nothing> {
        SqlxCase {
            kind: Kind::CheckConstraint,
            constraint,
            attrs: Nothing,
            _entity: PhantomData,
        }
    }

    pub const fn not_found<Entity>() -> SqlxCase<Entity, Nothing> {
        SqlxCase {
            kind: Kind::NotFound,
            constraint: "",
            attrs: Nothing,
            _entity: PhantomData,
        }
    }
}

impl<Entity: EntityTrait> SqlxCase<Entity, Nothing> {
    pub const fn with_attrs<Attrs: IntoIterator<Item = Entity::Attr>>(
        self,
        attrs: Attrs,
    ) -> SqlxCase<Entity, Attrs> {
        SqlxCase {
            kind: self.kind,
            constraint: self.constraint,
            attrs,
            _entity: self._entity,
        }
    }
}

impl<Entity: EntityTrait, Attrs: IntoIterator<Item = Entity::Attr>> SqlxCase<Entity, Attrs> {
    fn map(self, error: sqlx::Error) -> Either<sqlx::Error, Exception<Entity>> {
        match self.kind {
            Kind::NotFound => Either::Right(Exception::does_not_exist(self.attrs)),
            Kind::CheckConstraint
                if matches!(&error,
                    sqlx::Error::Database(db_err)
                        if db_err.is_check_violation() && db_err.constraint() == Some(self.constraint)
                ) =>
            {
                Either::Right(Exception::check_constraint_violation(self.attrs))
            }
            Kind::ForeignKeyConstraint
                if matches!(&error,
                    sqlx::Error::Database(db_err)
                        if db_err.is_foreign_key_violation() && db_err.constraint() == Some(self.constraint)
                ) =>
            {
                Either::Right(Exception::check_constraint_violation(self.attrs))
            }
            Kind::UniqueConstraint
                if matches!(&error,
                    sqlx::Error::Database(db_err)
                        if db_err.is_unique_violation() && db_err.constraint() == Some(self.constraint)
                ) =>
            {
                Either::Right(Exception::check_constraint_violation(self.attrs))
            }
            _ => Either::Left(error),
        }
    }
}

// use std::collections::HashMap;

// use anyhow::Context;
// use modql::SIden;
// use sea_query::{
//     backend::QueryBuilder, Asterisk, DynIden, Expr, IntoIden, Query, SimpleExpr, TableRef,
// };
// use sea_query_binder::{SqlxBinder, SqlxValues};
// use sqlx::{Database, Executor, IntoArguments, Row};

// use crate::{
//     entity::{AttrTrait, EntityTrait, Id},
//     outcome::Outcome,
// };

// use super::{ex::Exception, BaseRepo};

// pub trait DbConfig // where
// {
//     type QueryBuilder: QueryBuilder + Default;
//     type DbDriver: Database;
//     type Executor<'c>: Executor<'c, Database = Self::DbDriver>;
// }

// #[async_trait::async_trait]
// pub trait SqlxBaseRepo {
//     const TABLE: &'static str;

//     type DbConfig: DbConfig;
//     type Entity: EntityTrait + Send;

//     async fn executor(&self) -> <Self::DbConfig as DbConfig>::Executor<'_>;

//     fn attrs_values(
//         entity: Self::Entity,
//     ) -> HashMap<<Self::Entity as EntityTrait>::Attr, SimpleExpr>;

//     fn entity_from_row<'r, R: Row>(row: &'r R) -> sqlx::Result<Self::Entity>;

//     fn table_ref() -> TableRef {
//         TableRef::Table(SIden(Self::TABLE).into_iden())
//     }
// }

// #[async_trait::async_trait]
// impl<S: SqlxBaseRepo + Send + Sync> BaseRepo<S::Entity> for S
// where
//     for<'q> SqlxValues: IntoArguments<'q, <S::DbConfig as DbConfig>::DbDriver>,
//     <S::Entity as EntityTrait>::IdValue: Into<SimpleExpr> + Clone + Send + Sync,
//     S::Entity: Sync,
// {
//     async fn insert(&mut self, entity: S::Entity) -> Outcome<S::Entity, Exception<S::Entity>> {
//         let AttrsValues {
//             other_columns,
//             other_values,
//             ..
//         } = attrs_values::<Self>(entity)?;

//         let (sql, args) = Query::insert()
//             .into_table(S::table_ref())
//             .columns(other_columns)
//             .values_panic(other_values)
//             .returning_all()
//             .build_sqlx(<S::DbConfig as DbConfig>::QueryBuilder::default());

//         let row = sqlx::query_with::<<S::DbConfig as DbConfig>::DbDriver, _>(&sql, args)
//             .fetch_one(self.executor().await)
//             .await
//             .context("fialed to query insert sql")?;

//         let entity = S::entity_from_row(&row).context("failed to construct entity from sql row")?;
//         Outcome::Ok(entity)
//     }

//     async fn update(&mut self, entity: S::Entity) -> Outcome<S::Entity, Exception<S::Entity>> {
//         let AttrsValues {
//             id_column,
//             id_value,
//             other_columns,
//             other_values,
//         } = attrs_values::<Self>(entity)?;

//         let values = other_columns.into_iter().zip(other_values.into_iter());

//         let (sql, args) = Query::update()
//             .table(S::table_ref())
//             .values(values)
//             .and_where(Expr::col(id_column).is(id_value))
//             .returning_all()
//             .build_sqlx(<S::DbConfig as DbConfig>::QueryBuilder::default());

//         let row = sqlx::query_with::<<S::DbConfig as DbConfig>::DbDriver, _>(&sql, args)
//             .fetch_one(self.executor().await)
//             .await
//             .context("fialed to query insert sql")?;

//         let entity = S::entity_from_row(&row).context("failed to construct entity from sql row")?;
//         Outcome::Ok(entity)
//     }

//     async fn delete(&mut self, id: &Id<S::Entity>) -> Outcome<S::Entity, Exception<S::Entity>> {
//         let (id_column, id_value) = id_value(id);

//         let (sql, args) = Query::delete()
//             .from_table(S::table_ref())
//             .and_where(Expr::col(id_column).is(id_value))
//             .returning_all()
//             .build_sqlx(<S::DbConfig as DbConfig>::QueryBuilder::default());

//         let row = sqlx::query_with::<<S::DbConfig as DbConfig>::DbDriver, _>(&sql, args)
//             .fetch_one(self.executor().await)
//             .await
//             .context("fialed to query insert sql")?;

//         let entity = S::entity_from_row(&row).context("failed to construct entity from sql row")?;
//         Outcome::Ok(entity)
//     }

//     async fn find(&self, id: &Id<S::Entity>) -> Outcome<S::Entity, Exception<S::Entity>> {
//         let (id_column, id_value) = id_value(id);

//         let (sql, args) = Query::select()
//             .column(Asterisk)
//             .from(S::table_ref())
//             .and_where(Expr::col(id_column).is(id_value))
//             .build_sqlx(<S::DbConfig as DbConfig>::QueryBuilder::default());

//         let row = sqlx::query_with::<<S::DbConfig as DbConfig>::DbDriver, _>(&sql, args)
//             .fetch_one(self.executor().await)
//             .await
//             .context("fialed to query insert sql")?;

//         let entity = S::entity_from_row(&row).context("failed to construct entity from sql row")?;
//         Outcome::Ok(entity)
//     }
// }

// struct AttrsValues {
//     id_column: DynIden,
//     id_value: SimpleExpr,
//     other_columns: Vec<DynIden>,
//     other_values: Vec<SimpleExpr>,
// }

// fn attrs_values<R: SqlxBaseRepo>(entity: R::Entity) -> Result<AttrsValues, anyhow::Error> {
//     let id_attr = R::Entity::id_attr();
//     let mut values = R::attrs_values(entity);

//     let id_value = values
//         .remove(&id_attr)
//         .context("value of the id attr does not presented")?;

//     let (other_attrs, other_values): (Vec<_>, Vec<_>) = values.into_iter().unzip();

//     let id_column = DynIden::new(SIden(id_attr.name()));
//     let other_columns = other_attrs
//         .into_iter()
//         .map(|attr| DynIden::new(SIden(attr.name())))
//         .collect();

//     Ok(AttrsValues {
//         id_column,
//         id_value,
//         other_columns,
//         other_values,
//     })
// }

// fn id_value<Entity: EntityTrait>(id: &Id<Entity>) -> (DynIden, SimpleExpr)
// where
//     Entity::IdValue: Into<SimpleExpr> + Clone,
// {
//     let column = DynIden::new(SIden(Entity::id_attr().name()));
//     let value = id.value.clone().into();

//     (column, value)
// }
