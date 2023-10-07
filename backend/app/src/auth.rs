use anyhow::Context;
use domain::{Session, User};
use serde::{Deserialize, Serialize};

use super::{
    session::{DeleteSessionError, SaveSessionError, SessionService, ValidateSessionError},
    token::TokenService,
    user::{AuthenticateError, UserService},
};

pub struct AuthService<T> {
    user_serive: UserService<T>,
    session_service: SessionService<T>,
    token_service: TokenService,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error(transparent)]
    NewSessionError(#[from] NewSessionError),
    #[error(transparent)]
    AuthenticateError(#[from] AuthenticateError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenError {
    #[error(transparent)]
    NewSessionError(#[from] NewSessionError),
    #[error(transparent)]
    ValidateSessionError(#[from] ValidateSessionError),
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum NewSessionError {
    #[error(transparent)]
    SaveSessionError(#[from] SaveSessionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LogoutError {
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error(transparent)]
    ValidateSessionError(#[from] ValidateSessionError),
    #[error(transparent)]
    DeleteSessionError(#[from] DeleteSessionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl<T> AuthService<T> {
    pub fn new(
        session_service: SessionService<T>,
        user_serive: UserService<T>,
        token_service: TokenService,
    ) -> Self {
        Self {
            session_service,
            user_serive,
            token_service,
        }
    }

    pub async fn login(
        &self,
        tx: &mut T,
        login: &str,
        password: &str,
        session_metadata: String,
    ) -> Result<TokensPair, LoginError> {
        let user = self.user_serive.authenticate(tx, login, password).await?;
        let tokens_pair = self.new_session(tx, &user, session_metadata).await?;

        Ok(tokens_pair)
    }

    pub async fn refresh_token(
        &self,
        tx: &mut T,
        user_id: i32,
        refresh_token: &str,
        session_metadata: String,
    ) -> Result<TokensPair, RefreshTokenError> {
        let Some(user) = self.user_serive.get_user(tx, user_id).await? else {
            return Err(RefreshTokenError::UserDoesNotExist);
        };

        self.session_service
            .validate_refresh_token(tx, user.id, &session_metadata, &refresh_token)
            .await?;
        let tokens_pair = self.new_session(tx, &user, session_metadata).await?;

        Ok(tokens_pair)
    }

    pub async fn logout(
        &self,
        tx: &mut T,
        user_id: i32,
        refresh_token: &str,
        session_metadata: &str,
    ) -> Result<(), LogoutError> {
        let Some(user) = self.user_serive.get_user(tx, user_id).await? else {
            return Err(LogoutError::UserDoesNotExist);
        };

        let session = self
            .session_service
            .validate_refresh_token(tx, user.id, &session_metadata, &refresh_token)
            .await?;
        self.session_service.delete_session(tx, session).await?;

        Ok(())
    }

    async fn new_session(
        &self,
        tx: &mut T,
        user: &User,
        metadata: String,
    ) -> Result<TokensPair, NewSessionError> {
        let refresh_token = self
            .token_service
            .generate_refresh_token()
            .context("failed to generate refresh token")?;

        let Session { refresh_token, .. } = self
            .session_service
            .save_session(tx, user.id, metadata, refresh_token)
            .await?;

        let access_token = self
            .token_service
            .generate_access_token(&user)
            .context("failed to generate access token")?;

        Ok(TokensPair {
            refresh_token,
            access_token,
        })
    }
}
