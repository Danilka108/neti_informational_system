mod model;

use app::class_kind::{self, Entity, EntityId};
use sea_query::{Asterisk, Expr, Query};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{fetch_one, fetch_optional, PgTransaction};

use self::model::{ClassKinds, ClassKindsIden};

pub struct PgClassKindRepo {
    txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl PgClassKindRepo {
    async fn insert(&self, entity: Entity) -> Result<ClassKinds, anyhow::Error> {
        let mut query = Query::insert();
        let query = query
            .into_table(ClassKindsIden::Table)
            .columns([ClassKindsIden::Name])
            .values_panic([entity.name.value.into()])
            .returning_all();

        fetch_one(&self.txn, query).await
    }

    async fn update(&self, entity: Entity) -> Result<ClassKinds, anyhow::Error> {
        let mut query = Query::update();
        let query = query
            .table(ClassKindsIden::Table)
            .values([(ClassKindsIden::Name, entity.name.value.into())])
            .returning_all();

        fetch_one(&self.txn, query).await
    }
}

#[async_trait::async_trait]
impl class_kind::Repo for PgClassKindRepo {
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
                .from_table(ClassKindsIden::Table)
                .and_where(Expr::col(ClassKindsIden::Table).is(entity.name.value.clone())),
        )
        .await?;

        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let model = fetch_optional::<ClassKinds>(
            &self.txn,
            Query::select()
                .from(ClassKindsIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(ClassKindsIden::Table).is(id.value)),
        )
        .await?;

        Ok(model.map(Into::into))
    }
}
