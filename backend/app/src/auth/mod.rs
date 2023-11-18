use std::num::NonZeroI32;

use crate::{
    session::{
        DeleteSessionException, SaveSessionException, Session, SessionService,
        UpdateSessionException,
    },
    token::{TokenService, Tokens},
    user::{AuthenticateUserException, UserService},
    Outcome,
};

pub struct AuthService {
    pub(crate) user_service: UserService,
    pub(crate) session_service: SessionService,
    pub(crate) token_service: TokenService,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginException {
    #[error(transparent)]
    FailedToSaveSession(#[from] SaveSessionException),
    #[error(transparent)]
    FailedToAuthenticate(#[from] AuthenticateUserException),
}

impl AuthService {
    pub async fn login(
        self,
        email: &str,
        password: &str,
        session_metadata: String,
    ) -> Outcome<Tokens, LoginException> {
        let user = self.user_service.authenticate(email, password).await?;

        let Tokens {
            access_token,
            refresh_token,
        } = self.token_service.generate(&user).await?;

        let Session { refresh_token, .. } = self
            .session_service
            .save(*user.id, session_metadata, refresh_token)
            .await?;

        Outcome::Success(Tokens {
            access_token,
            refresh_token,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenException {
    #[error(transparent)]
    FailedToUpdateSession(#[from] UpdateSessionException),
    #[error("user does not exist")]
    UserDoesNotExist,
}

impl AuthService {
    pub async fn refresh_token(
        self,
        user_id: NonZeroI32,
        refresh_token_to_validate: &str,
        session_metadata: String,
    ) -> Outcome<Tokens, RefreshTokenException> {
        let Some(user) = self.user_service.find_by_id(user_id).await? else {
            return Outcome::Exception(RefreshTokenException::UserDoesNotExist);
        };

        let Tokens {
            access_token,
            refresh_token: new_refresh_token,
        } = self.token_service.generate(&user).await?;

        let Session { refresh_token, .. } = self
            .session_service
            .update(
                user_id,
                session_metadata,
                refresh_token_to_validate,
                new_refresh_token,
            )
            .await?;

        Outcome::Success(Tokens {
            access_token,
            refresh_token,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LogoutException {
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error(transparent)]
    FailedToDeleteSession(#[from] DeleteSessionException),
}

impl AuthService {
    pub async fn logout(
        self,
        user_id: NonZeroI32,
        refresh_token_to_validate: &str,
        session_metadata: &str,
    ) -> Outcome<(), LogoutException> {
        let Some(_user) = self.user_service.find_by_id(user_id).await? else {
            return Outcome::Exception(LogoutException::UserDoesNotExist);
        };

        let _deleted_session = self
            .session_service
            .delete(user_id, session_metadata, refresh_token_to_validate)
            .await;

        Outcome::Success(())
    }
}
