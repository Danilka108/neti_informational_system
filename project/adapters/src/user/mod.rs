mod models;

use app::user::{self, Entity, EntityId};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    fetch_one, fetch_optional,
    user::models::{Users, UsersIden},
    PgTransaction,
};

pub struct PgUserRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgUserRepo {
    async fn update(&self, entity: Entity) -> Result<Users, anyhow::Error> {
        let mut query = Query::update();
        let query = query
            .table(UsersIden::Table)
            .values([
                (UsersIden::Email, entity.email.into()),
                (UsersIden::Password, entity.password.value.into()),
            ])
            .and_where(Expr::col(UsersIden::Id).is(entity.id.value))
            .returning_all();

        fetch_one(&self.txn, query).await
    }

    async fn insert(&self, entity: Entity) -> Result<Users, anyhow::Error> {
        let mut query = Query::insert();

        let query = query
            .into_table(UsersIden::Table)
            .columns([UsersIden::Email, UsersIden::Password])
            .values_panic([entity.email.into(), entity.password.value.into()])
            .returning_all();

        fetch_one(&self.txn, query).await
    }
}

#[async_trait::async_trait]
impl user::Repo for PgUserRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id).await?.is_some() {
            self.update(entity).await?
        } else {
            self.insert(entity).await?
        };

        Ok(model.into())
    }

    async fn delete(&mut self, entity: &Entity) -> Result<(), anyhow::Error> {
        fetch_one::<()>(
            &self.txn,
            Query::delete()
                .from_table(UsersIden::Table)
                .and_where(Expr::col(UsersIden::Id).is(entity.id.value)),
        )
        .await?;

        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let res = fetch_optional::<Users>(
            &self.txn,
            Query::select()
                .from(UsersIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(UsersIden::Id).is(id.value)),
        )
        .await?;

        Ok(res.map(Into::into))
    }

    async fn find_by_email(&self, email: String) -> Result<Option<Entity>, anyhow::Error> {
        let res = fetch_optional::<Users>(
            &self.txn,
            Query::select()
                .from(UsersIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(UsersIden::Email).is(email)),
        )
        .await?;

        Ok(res.map(Into::into))
    }
}
