use crate::{
    session::{DeleteSessionError, SaveSessionError, Session, SessionService, UpdateSessionError},
    token::{TokenService, Tokens},
    user::{AuthenticateUserError, UserService},
};

pub struct AuthService {
    pub(crate) user_service: UserService,
    pub(crate) session_service: SessionService,
    pub(crate) token_service: TokenService,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error(transparent)]
    SaveSessionError(#[from] SaveSessionError),
    #[error(transparent)]
    AuthenticateError(#[from] AuthenticateUserError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AuthService {
    pub async fn login(
        self,
        email: &str,
        password: &str,
        session_metadata: String,
    ) -> Result<Tokens, LoginError> {
        let user = self.user_service.authenticate(email, password).await?;

        let Tokens {
            access_token,
            refresh_token,
        } = self.token_service.generate(&user).await?;

        let Session { refresh_token, .. } = self
            .session_service
            .save(user.id, session_metadata, refresh_token)
            .await?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenError {
    #[error(transparent)]
    UpdateSessionError(#[from] UpdateSessionError),
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AuthService {
    pub async fn refresh_token(
        self,
        user_id: i32,
        refresh_token_to_validate: &str,
        session_metadata: String,
    ) -> Result<Tokens, RefreshTokenError> {
        let Some(user) = self.user_service.find_by_id(user_id).await? else {
            return Err(RefreshTokenError::UserDoesNotExist);
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

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }
}

pub struct Logout<'r, 'm> {
    pub user_id: i32,
    pub refresh_token: &'r str,
    pub session_metadata: &'m str,
}

#[derive(Debug, thiserror::Error)]
pub enum LogoutError {
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error(transparent)]
    DeleteSessionError(#[from] DeleteSessionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AuthService {
    pub async fn logout(
        self,
        user_id: i32,
        refresh_token_to_validate: &str,
        session_metadata: &str,
    ) -> Result<(), LogoutError> {
        let Some(_user) = self.user_service.find_by_id(user_id).await? else {
            return Err(LogoutError::UserDoesNotExist);
        };

        let _deleted_session = self
            .session_service
            .delete(user_id, session_metadata, refresh_token_to_validate)
            .await;

        Ok(())
    }
}
