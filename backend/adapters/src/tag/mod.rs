mod model;

use app::tag::{self, Entity, EntityId};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fetch_one, fetch_optional, PgTransaction};

use self::model::{Tags, TagsIden};

pub struct PgTagRepo {
    pub txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgTagRepo {
    async fn insert(&self, entity: Entity) -> Result<Tags, anyhow::Error> {
        let mut query = Query::insert();
        let query = query
            .into_table(TagsIden::Table)
            .columns([TagsIden::Name])
            .values_panic([entity.name.value.into()])
            .returning_all();

        fetch_one(&self.txn, query).await
    }

    async fn update(&self, entity: Entity) -> Result<Tags, anyhow::Error> {
        let mut query = Query::update();
        let query = query
            .table(TagsIden::Table)
            .values([(TagsIden::Name, entity.name.value.into())])
            .returning_all();

        fetch_one(&self.txn, query).await
    }
}

#[async_trait::async_trait]
impl tag::Repo for PgTagRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.name.clone()).await?.is_some() {
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
                .from_table(TagsIden::Table)
                .and_where(Expr::col(TagsIden::Table).eq(entity.name.value.clone())),
        )
        .await?;

        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let model = fetch_optional::<Tags>(
            &self.txn,
            Query::select()
                .from(TagsIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(TagsIden::Table).eq(id.value)),
        )
        .await?;

        Ok(model.map(Into::into))
    }
}
