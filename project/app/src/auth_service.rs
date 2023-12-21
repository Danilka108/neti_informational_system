use utils::{di::Provide, outcome::Outcome};

use crate::{
    token::{self, BoxedAccessTokenEngine, BoxedRefreshTokenGenerator},
    user,
    user_session::{self, SecondsFromUnixEpoch},
    AdaptersModule, AppModule,
};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error(transparent)]
    UserException(#[from] user::Exception),
    #[error(transparent)]
    SessionException(#[from] user_session::Exception),
}

pub struct AuthService<A> {
    ctx: AppModule<A>,
}

impl<A: AdaptersModule + Clone> Provide<AuthService<A>> for AppModule<A> {
    fn provide(&self) -> AuthService<A> {
        AuthService { ctx: self.clone() }
    }
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl<A: AdaptersModule + Clone + Sync> AuthService<A> {
    pub async fn login(
        self,
        email: String,
        password: String,
        session_metadata: String,
    ) -> Outcome<Tokens, Exception> {
        let refersh_token_generator: BoxedRefreshTokenGenerator = self.ctx.adapters.resolve();
        let token::AccessTokenTTL(access_token_ttl) = self.ctx.adapters.resolve();
        let access_token_engine: BoxedAccessTokenEngine = self.ctx.adapters.resolve();

        let user = user::Entity::get_by_email(email)
            .exec(self.ctx.clone())
            .await?;

        user.validate_password(&password)
            .exec(self.ctx.clone())
            .await?;

        let refresh_token = refersh_token_generator.generate().await?;

        let access_token = access_token_engine
            .encode(token::Claims {
                user_id: user.id().value,
                email: user.email().to_owned(),
                expires_at: SecondsFromUnixEpoch::expired_at_from_ttl(access_token_ttl)?,
            })
            .await?;

        let session = user_session::Entity::save(user.id(), session_metadata, refresh_token)
            .exec(self.ctx.clone())
            .await?;

        Outcome::Ok(Tokens {
            access_token,
            refresh_token: session.refresh_token,
        })
    }

    pub async fn refresh_token(
        self,
        user_id: i32,
        refresh_token_to_validate: String,
        session_metadata: String,
    ) -> Outcome<Tokens, Exception> {
        let refresh_token_generator: BoxedRefreshTokenGenerator = self.ctx.adapters.resolve();
        let access_token_engine: BoxedAccessTokenEngine = self.ctx.adapters.resolve();
        let token::AccessTokenTTL(access_token_ttl) = self.ctx.adapters.resolve();

        let user = user::Entity::get(user_id).exec(self.ctx.clone()).await?;

        let refresh_token = refresh_token_generator.generate().await?;
        let access_token = access_token_engine
            .encode(token::Claims {
                user_id,
                email: user.email().to_owned(),
                expires_at: SecondsFromUnixEpoch::expired_at_from_ttl(access_token_ttl)?,
            })
            .await?;

        let session = user_session::Entity::update(
            user.id(),
            session_metadata,
            refresh_token_to_validate,
            refresh_token,
        )
        .exec(self.ctx.clone())
        .await?;

        Outcome::Ok(Tokens {
            access_token,
            refresh_token: session.refresh_token,
        })
    }

    pub async fn logout(
        self,
        user_id: i32,
        refresh_token_to_validate: String,
        session_metadata: String,
    ) -> Outcome<(), Exception> {
        let user = user::Entity::get(user_id).exec(self.ctx.clone()).await?;

        let _deleted_session =
            user_session::Entity::remove(user.id(), session_metadata, refresh_token_to_validate)
                .exec(self.ctx.clone())
                .await?;

        Outcome::Ok(())
    }
}
