mod model;
use app::{
    user,
    user_session::{self, Entity, EntityId},
};
use sea_query::{Asterisk, Expr, Func, Query};
use tokio::sync::Mutex;

use self::model::{UserSessions, UserSessionsIden};
use crate::{fetch_all, fetch_one, fetch_optional, PgTransaction};

pub struct PgUserSessionRepo {
    pub txn: std::sync::Arc<Mutex<PgTransaction<'static>>>,
}

impl PgUserSessionRepo {
    async fn insert(&self, entity: Entity) -> Result<UserSessions, anyhow::Error> {
        let mut query = Query::insert();
        let query = query
            .into_table(UserSessionsIden::Table)
            .columns([
                UserSessionsIden::UserId,
                UserSessionsIden::Metadata,
                UserSessionsIden::RefreshToken,
                UserSessionsIden::ExpiresAt,
            ])
            .values_panic([
                entity.id.value.user_id.value.into(),
                entity.id.value.metadata.into(),
                entity.refresh_token.into(),
                entity.expires_at.seconds.val.into(),
            ])
            .returning_all();

        fetch_one(&self.txn, query).await
    }

    async fn update(&self, entity: Entity) -> Result<UserSessions, anyhow::Error> {
        let mut query = Query::update();
        let query = query
            .table(UserSessionsIden::Table)
            .values([
                (
                    UserSessionsIden::UserId,
                    entity.id.value.user_id.value.into(),
                ),
                (UserSessionsIden::Metadata, entity.id.value.metadata.into()),
                (UserSessionsIden::RefreshToken, entity.refresh_token.into()),
                (
                    UserSessionsIden::ExpiresAt,
                    entity.expires_at.seconds.val.into(),
                ),
            ])
            .returning_all();

        fetch_one(&self.txn, query).await
    }
}

#[async_trait::async_trait]
impl user_session::Repo for PgUserSessionRepo {
    async fn save(&mut self, entity: Entity) -> Result<Entity, anyhow::Error> {
        let model = if self.find(entity.id.clone()).await?.is_some() {
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
                .from_table(UserSessionsIden::Table)
                .and_where(
                    Expr::col(UserSessionsIden::UserId)
                        .eq(entity.id.value.user_id.value)
                        .and(
                            Expr::col(UserSessionsIden::Metadata).eq(entity
                                .id
                                .value
                                .metadata
                                .clone()),
                        ),
                ),
        )
        .await?;

        Ok(())
    }

    async fn find(&self, id: EntityId) -> Result<Option<Entity>, anyhow::Error> {
        let cond = Expr::col(UserSessionsIden::UserId)
            .eq(id.value.user_id.value)
            .and(Expr::col(UserSessionsIden::Metadata).eq(id.value.metadata.clone()));

        let res = fetch_optional::<UserSessions>(
            &self.txn,
            Query::select()
                .from(UserSessionsIden::Table)
                .column(Asterisk)
                .and_where(cond),
        )
        .await?
        .map(Into::into);

        Ok(res)
    }

    async fn list_by_user_id(&self, user_id: user::EntityId) -> Result<Vec<Entity>, anyhow::Error> {
        let res = fetch_all::<UserSessions>(
            &self.txn,
            Query::select()
                .from(UserSessionsIden::Table)
                .column(Asterisk)
                .and_where(Expr::col(UserSessionsIden::UserId).eq(user_id.value)),
        )
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

        Ok(res)
    }

    async fn count_not_expired(&self, user_id: user::EntityId) -> Result<i64, anyhow::Error> {
        let (res,): (i64,) = fetch_one(
            &self.txn,
            Query::select()
                .from(UserSessionsIden::Table)
                .expr(Func::count(Expr::col(Asterisk)))
                .and_where(Expr::col(UserSessionsIden::UserId).eq(user_id.value)),
        )
        .await?;

        Ok(res)
    }
}
