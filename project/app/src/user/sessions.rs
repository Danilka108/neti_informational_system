use utils::{entity::entity_method, outcome::Outcome};

use crate::{AdaptersModule, AppModule};

use super::{
    BoxedRepo, Entity, Exception, SecondsFromUnixEpoch, Session, SessionTTL, SessionsMaxNumber,
};

#[derive(Debug, Clone)]
pub struct Sessions<'e> {
    pub(super) entity: &'e Entity,
}

impl<'e> Sessions<'e> {
    #[entity_method(ctx)]
    async fn get_validated<A: AdaptersModule>(
        &self,
        ctx: AppModule<A>,
        metadata: String,
        refresh_token_to_validate: String,
    ) -> Outcome<Session, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();

        let session = match repo
            .find_session(&self.entity.id, metadata)
            .await
            .map_repo_ex()
        {
            Outcome::Ok(session) => session,
            Outcome::Ex(Exception::SessionDoesNotExist) => {
                let _deleted_sessions = repo
                    .delete_all_sessions(&self.entity.id)
                    .await
                    .map_repo_ex()?;
                return Outcome::Ex(Exception::SessionDoesNotExist);
            }
            Outcome::Ex(ex) => return Outcome::Ex(ex),
            Outcome::Error(err) => return Outcome::Error(err),
        };

        if session.refresh_token != refresh_token_to_validate {
            return Outcome::Ex(Exception::InvalidRefreshToken);
        }

        if session.expires_at.is_expired()? {
            let _deleted_sessions = repo
                .delete_all_sessions(&self.entity.id)
                .await
                .map_repo_ex()?;
            return Outcome::Ex(Exception::SessionExpired);
        }

        Outcome::Ok(session)
    }

    #[entity_method(ctx)]
    pub async fn update<A: AdaptersModule + Clone + Sync>(
        &self,
        ctx: AppModule<A>,
        metadata: String,
        refresh_token_to_validate: String,
        new_refresh_token: String,
    ) -> Outcome<Session, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();

        let old_session = self
            .get_validated(metadata, refresh_token_to_validate)
            .exec(ctx.clone())
            .await?;

        let new_session = Session {
            refresh_token: new_refresh_token,
            ..old_session
        };

        repo.update_session(&self.entity.id, new_session)
            .await
            .map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn save<A: AdaptersModule + Clone + Sync + 'e>(
        self,
        ctx: AppModule<A>,
        metadata: String,
        refresh_token: String,
    ) -> Outcome<Session, Exception> {
        let SessionTTL(ttl) = ctx.adapters.resolve();
        let SessionsMaxNumber(sessions_max_number) = ctx.adapters.resolve();
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();

        let expires_at = SecondsFromUnixEpoch::expired_at_from_ttl(ttl)?;

        let session = Session {
            metadata: metadata.clone(),
            refresh_token,
            expires_at,
        };

        if self
            .get(metadata)
            .exec(ctx.clone())
            .await
            .into_result()?
            .is_ok()
        {
            return repo
                .update_session(&self.entity.id, session)
                .await
                .map_repo_ex();
        }

        let sessions_number = self.count_not_expired().exec(ctx.clone()).await?;
        if sessions_number >= sessions_max_number {
            return Outcome::Ex(Exception::SessionsLimitReached);
        }

        repo.add_session(&self.entity.id, session)
            .await
            .map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn remove<A: AdaptersModule + Clone + Sync>(
        &self,
        ctx: AppModule<A>,
        metadata: String,
        refresh_token_to_validate: String,
    ) -> Outcome<Session, Exception> {
        let mut repo = ctx.adapters.resolve::<BoxedRepo>();

        let _session = self
            .get_validated(metadata.clone(), refresh_token_to_validate)
            .exec(ctx.clone())
            .await?;

        repo.remove_session(&self.entity.id, metadata)
            .await
            .map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn get<A: AdaptersModule>(
        &self,
        ctx: AppModule<A>,
        metadata: String,
    ) -> Outcome<Session, Exception> {
        let repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.find_session(&self.entity.id, metadata)
            .await
            .map_repo_ex()
    }

    #[entity_method(ctx)]
    pub async fn get_all<A: AdaptersModule>(
        &self,
        ctx: AppModule<A>,
    ) -> Outcome<Vec<Session>, Exception> {
        let repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.list_sessions(&self.entity.id).await.map_repo_ex()
    }

    #[entity_method(ctx)]
    async fn count_not_expired<A: AdaptersModule>(
        &self,
        ctx: AppModule<A>,
    ) -> Outcome<usize, Exception> {
        let repo = ctx.adapters.resolve::<BoxedRepo>();
        repo.count_not_expired_sessions(&self.entity.id)
            .await
            .map_repo_ex()
    }

    // #[entity_method(ctx)]
    // pub async fn add_member<A: AdaptersModule>(
    //     self,
    //     ctx: AppModule<A>,
    //     member: Member,
    // ) -> Outcome<Member, Exception> {
    //     let mut repo = ctx.adapters.resolve::<BoxedRepo>();
    //     repo.add_member(&self.id, member).await.map_repo_ex()
    // }

    // #[entity_method(ctx)]
    // pub async fn remove_member<A: AdaptersModule>(
    //     self,
    //     ctx: AppModule<A>,
    //     person_id: impl ProvideId<person::Entity> + Send + Sync,
    // ) -> Outcome<Member, Exception> {
    //     let mut repo = ctx.adapters.resolve::<BoxedRepo>();
    //     repo.remove_member(&self.id, person_id.provide_id())
    //         .await
    //         .map_repo_ex()
    // }

    // #[entity_method(ctx)]
    // pub async fn get_member<A: AdaptersModule>(
    //     self,
    //     ctx: AppModule<A>,
    //     person_id: impl ProvideId<person::Entity> + Send + Sync,
    // ) -> Outcome<Member, Exception> {
    //     let repo = ctx.adapters.resolve::<BoxedRepo>();
    //     repo.find_member(&self.id, person_id.provide_id())
    //         .await
    //         .map_repo_ex()
    // }

    // #[entity_method(ctx)]
    // pub async fn get_members<A: AdaptersModule>(
    //     self,
    //     ctx: AppModule<A>,
    // ) -> Outcome<Vec<Member>, Exception> {
    //     let repo = ctx.adapters.resolve::<BoxedRepo>();
    //     repo.list_members(&self.id).await.map_repo_ex()
    // }
}
