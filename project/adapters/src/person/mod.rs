mod models;

use app::user::{Entity, EntityAttr};
use sea_query::{Asterisk, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use std::sync::Arc;
use tokio::sync::Mutex;
use utils::{
    entity::Id,
    outcome::Outcome,
    repo::{
        ex::Exception,
        sqlx::{IntoSqlxMapper, SqlxCase},
        BaseRepo,
    },
};

use self::models::Persons;
use crate::{person::models::PersonsIden, PgTransaction};

pub struct PgPersonRepository {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

#[async_trait::async_trait]
impl BaseRepo<Entity> for PgPersonRepository {
    async fn insert(&mut self, entity: Entity) -> Outcome<Entity, Exception<Entity>> {
        let (sql, args) = Query::insert()
            .into_table(PersonsIden::Table)
            .columns([PersonsIden::Email, PersonsIden::Password])
            .values_panic([entity.email.into(), entity.password.value.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let person: Persons = sqlx::query_as_with(&sql, args)
            .fetch_one(self.txn.lock().await.as_mut())
            .await
            .into_sqlx_mapper()
            .case(SqlxCase::unique_constraint("persons_email_key").with_attrs([EntityAttr::Email]))
            .map()?;

        Outcome::Ok(person.into())
    }

    async fn update(&mut self, entity: Entity) -> Outcome<Entity, Exception<Entity>> {
        let (sql, args) = Query::update()
            .table(PersonsIden::Table)
            .values([
                (PersonsIden::Email, entity.email.into()),
                (PersonsIden::Password, entity.password.value.into()),
            ])
            .and_where(Expr::col(PersonsIden::Id).is(entity.id.value))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let person: Persons = sqlx::query_as_with(&sql, args)
            .fetch_one(self.txn.lock().await.as_mut())
            .await
            .into_sqlx_mapper()
            .case(SqlxCase::unique_constraint("persons_email_key").with_attrs([EntityAttr::Email]))
            .map()?;

        Outcome::Ok(person.into())
    }

    async fn delete(&mut self, id: &Id<Entity>) -> Outcome<Entity, Exception<Entity>> {
        let (sql, args) = Query::delete()
            .from_table(PersonsIden::Table)
            .and_where(Expr::col(PersonsIden::Id).is(id.value))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let person: Persons = sqlx::query_as_with(&sql, args)
            .fetch_one(self.txn.lock().await.as_mut())
            .await
            .into_sqlx_mapper()
            .case(SqlxCase::unique_constraint("persons_email_key").with_attrs([EntityAttr::Email]))
            .map()?;

        Outcome::Ok(person.into())
    }

    async fn find(&self, id: &Id<Entity>) -> Outcome<Entity, Exception<Entity>> {
        let (sql, args) = Query::select()
            .from(PersonsIden::Table)
            .column(Asterisk)
            .and_where(Expr::col(PersonsIden::Id).is(id.value))
            .build_sqlx(PostgresQueryBuilder);

        let person: Persons = sqlx::query_as_with(&sql, args)
            .fetch_one(self.txn.lock().await.as_mut())
            .await
            .into_sqlx_mapper()
            .case(SqlxCase::unique_constraint("persons_email_key").with_attrs([EntityAttr::Email]))
            .map()?;

        Outcome::Ok(person.into())
    }
}
