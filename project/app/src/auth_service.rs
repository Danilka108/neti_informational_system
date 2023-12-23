use utils::{
    di::{Module, Provide},
    entity::Id,
    outcome::Outcome,
};

use crate::{
    token::{AccessTokenTTL, BoxedAccessTokenEngine, Claims},
    user_service::{UserException, UserService},
    user_session::SecondsFromUnixEpoch,
    AdaptersModule, AppModule,
};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error(transparent)]
    UserException(#[from] UserException),
}

pub struct AuthService {
    user_service: UserService,
    access_token_engine: BoxedAccessTokenEngine,
    access_token_ttl: AccessTokenTTL,
}

impl<A: AdaptersModule> Provide<AuthService> for AppModule<A> {
    fn provide(&self) -> AuthService {
        AuthService {
            user_service: self.resolve(),
            access_token_engine: self.adapters.resolve(),
            access_token_ttl: self.adapters.resolve(),
        }
    }
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl AuthService {
    pub async fn login(
        &mut self,
        email: String,
        password: String,
        session_metadata: String,
    ) -> Outcome<Tokens, Exception> {
        let AccessTokenTTL(access_token_ttl) = self.access_token_ttl;

        let user = self.user_service.authenticate(email, password).await?;
        let session = self
            .user_service
            .create_session(user.id, session_metadata)
            .await?;

        let access_token = self
            .access_token_engine
            .encode(Claims {
                user_id: user.id.value,
                email: user.email,
                expires_at: SecondsFromUnixEpoch::expired_at_from_ttl(access_token_ttl)?,
            })
            .await?;

        Outcome::Ok(Tokens {
            access_token,
            refresh_token: session.refresh_token,
        })
    }

    pub async fn refresh_token(
        &mut self,
        user_id: i32,
        refresh_token_to_validate: String,
        session_metadata: String,
    ) -> Outcome<Tokens, Exception> {
        let AccessTokenTTL(access_token_ttl) = self.access_token_ttl;

        let user = self.user_service.get(Id::new(user_id)).await?;
        let session = self
            .user_service
            .update_session(user.id, session_metadata, refresh_token_to_validate)
            .await?;

        let access_token = self
            .access_token_engine
            .encode(Claims {
                user_id: user.id.value,
                email: user.email,
                expires_at: SecondsFromUnixEpoch::expired_at_from_ttl(access_token_ttl)?,
            })
            .await?;

        Outcome::Ok(Tokens {
            access_token,
            refresh_token: session.refresh_token,
        })
    }

    pub async fn logout(
        &mut self,
        user_id: i32,
        session_metadata: String,
        refresh_token_to_validate: String,
    ) -> Outcome<(), Exception> {
        let user = self.user_service.get(Id::new(user_id)).await?;
        let _removed_session = self
            .user_service
            .remove_session(user.id, session_metadata, refresh_token_to_validate)
            .await?;

        Outcome::Ok(())
    }
}
