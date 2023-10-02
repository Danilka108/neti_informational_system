use std::time::{Duration, SystemTime};

use anyhow::Context;
use domain::{AuthClaims, Session, User};
use serde::{Deserialize, Serialize};

use crate::api::{PasswordEncoder, SessionRepository, UserRepository};

use super::user::{self, AuthenticateError};

pub struct AuthService<SessionRepo, UserRepo, TokenManager, PassEncoder> {
    session_repo: SessionRepo,
    user_serive: user::UserService<UserRepo, PassEncoder>,
    token_manager: TokenManager,
    max_number_of_sessions: usize,
    session_ttl_in_seconds: u64,
    jwt_token_ttl_in_seconds: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("the limit on the sessions number has been reached, the maximum number of sessions is {}", .limit)]
    SessionsLimitReached { limit: usize },
    #[error(transparent)]
    AuthenticateError(#[from] AuthenticateError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenError {
    #[error("invalid session")]
    InvalidSession,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LogoutError {
    #[error("invalid session")]
    InvalidSession,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl<Transaction, SessionRepo, UserRepo, TokenManager, PassEncoder>
    AuthService<SessionRepo, UserRepo, TokenManager, PassEncoder>
where
    SessionRepo: SessionRepository<Transaction = Transaction>,
    UserRepo: UserRepository<Transaction = Transaction>,
    TokenManager: crate::api::TokenManager,
    PassEncoder: PasswordEncoder,
{
    pub fn new(
        session_repo: SessionRepo,
        user_serive: user::UserService<UserRepo, PassEncoder>,
        jwt_encoder: TokenManager,
        max_number_of_sessions: usize,
        session_ttl_in_seconds: u64,
        jwt_token_ttl_in_seconds: u64,
    ) -> Self {
        Self {
            session_repo,
            user_serive,
            token_manager: jwt_encoder,
            max_number_of_sessions,
            session_ttl_in_seconds,
            jwt_token_ttl_in_seconds,
        }
    }

    pub async fn login(
        &mut self,
        transaction: &mut Transaction,
        login: &str,
        password: &str,
        session_metadata: String,
    ) -> Result<TokensPair, LoginError> {
        let user = self
            .user_serive
            .authenticate(transaction, login, password)
            .await?;

        if self
            .is_session_already_present(transaction, &user, &session_metadata)
            .await?
        {
            self.check_sessions_limit(transaction, &user).await?;
        }

        let Session { refresh_token, .. } = self
            .save_session(transaction, &user, session_metadata)
            .await?;

        let access_token = self.generate_access_token(&user)?;

        Ok(TokensPair {
            refresh_token,
            access_token,
        })
    }

    async fn check_sessions_limit(
        &self,
        transaction: &mut Transaction,
        user: &User,
    ) -> Result<(), LoginError> {
        let sessions_number = self
            .session_repo
            .count_by_user_id(transaction, user.id)
            .await
            .context("failed to read session repository")?;

        if sessions_number >= self.max_number_of_sessions {
            Err(LoginError::SessionsLimitReached {
                limit: self.max_number_of_sessions,
            })
        } else {
            Ok(())
        }
    }

    pub async fn refresh_token(
        &mut self,
        transaction: &mut Transaction,
        refresh_token: &str,
        metadata: String,
    ) -> Result<TokensPair, RefreshTokenError> {
        let Some(session) = self
            .check_session(transaction, refresh_token, &metadata)
            .await?
        else {
            return Err(RefreshTokenError::InvalidSession);
        };

        let user = self
            .user_serive
            .get_user(transaction, session.user_id)
            .await?
            .context("failed to get user by user id")?;

        let Session { refresh_token, .. } = self.save_session(transaction, &user, metadata).await?;
        let access_token = self.generate_access_token(&user)?;

        Ok(TokensPair {
            access_token,
            refresh_token,
        })
    }

    pub async fn logout(
        &mut self,
        transaction: &mut Transaction,
        refresh_token: &str,
        metadata: &str,
    ) -> Result<(), LogoutError> {
        let Some(session) = self
            .check_session(transaction, refresh_token, metadata)
            .await?
        else {
            return Err(LogoutError::InvalidSession);
        };

        let _ = self
            .session_repo
            .delete(transaction, session)
            .await
            .context("failed to delete in session repository")?;

        Ok(())
    }

    async fn check_session(
        &mut self,
        transaction: &mut Transaction,
        refresh_token: &str,
        metadata: &str,
    ) -> Result<Option<Session>, anyhow::Error> {
        let res = self
            .session_repo
            .find_by_metadata_and_token(transaction, refresh_token, &metadata)
            .await
            .context("failed to read from session repository")?
            .filter(|s| s.refresh_token == refresh_token);

        Ok(res)
    }

    async fn is_session_already_present(
        &self,
        transaction: &mut Transaction,
        user: &User,
        metadata: &str,
    ) -> Result<bool, anyhow::Error> {
        let result = self
            .session_repo
            .find(transaction, user.id, metadata)
            .await
            .context("failed to read session repository")?
            .is_some();

        Ok(result)
    }

    async fn save_session(
        &mut self,
        transaction: &mut Transaction,
        user: &User,
        metadata: String,
    ) -> Result<Session, anyhow::Error> {
        let refresh_token = self
            .token_manager
            .generate_refresh_token()
            .context("failed to generate refresh token")?;

        let session = Session {
            user_id: user.id,
            metadata,
            refresh_token,
            ttl_in_seconds: self.session_ttl_in_seconds,
        };

        let session = self
            .session_repo
            .save(transaction, session)
            .await
            .context("failed to save to session repository")?;

        Ok(session)
    }

    fn generate_access_token(&self, user: &User) -> Result<String, anyhow::Error> {
        let issued_at_unix_epoch_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("failed to get duration since unix epoch")?
            .checked_add(Duration::from_secs(self.jwt_token_ttl_in_seconds))
            .context("failed to compute issuted at for jwt token")?
            .as_secs();

        let claims = AuthClaims {
            user_id: user.id,
            issued_at_unix_epoch_secs,
            role: user.role,
        };

        self.token_manager
            .encode_jwt_token(claims)
            .context("failed to encode jwt token")
    }
}
