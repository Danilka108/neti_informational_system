use anyhow::Context;

use super::{DynSessionRepository, Session, SessionTTL, SessionsMaxNumber};

pub struct SessionService {
    pub(crate) repo: DynSessionRepository,
    pub(crate) session_ttl: SessionTTL,
    pub(crate) sessions_max_number: SessionsMaxNumber,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateSessionError {
    #[error("no session found")]
    NoSessionFound,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("session expired")]
    SessionExpired,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SessionService {
    async fn validate(
        &self,
        user_id: i32,
        metadata: &str,
        refresh_token: &str,
    ) -> Result<Session, ValidateSessionError> {
        let Some(session) = self.repo.find(user_id, metadata).await? else {
            self.repo.delete_all(user_id).await?;
            return Err(ValidateSessionError::NoSessionFound);
        };

        if session.refresh_token != refresh_token {
            self.repo.delete_all(user_id).await?;
            return Err(ValidateSessionError::InvalidRefreshToken);
        }

        if session.is_expired()? {
            self.repo.delete_all(user_id).await?;
            return Err(ValidateSessionError::SessionExpired);
        }

        Ok(session)
    }

    async fn is_session_exist(&self, session: &Session) -> Result<bool, anyhow::Error> {
        let result = self
            .repo
            .find(session.user_id, &session.metadata)
            .await?
            .is_some();

        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSessionError {
    #[error(transparent)]
    ValidateError(#[from] ValidateSessionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SessionService {
    pub(crate) async fn delete(
        self,
        user_id: i32,
        metadata: &str,
        refresh_token_to_validate: &str,
    ) -> Result<Session, DeleteSessionError> {
        let _ = self
            .validate(user_id, metadata, refresh_token_to_validate)
            .await?;

        let deleted_session =
            self.repo.delete(user_id, metadata).await?.context(
                "the session existence was checked before deleting it, but an error occurs",
            )?;

        Ok(deleted_session)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateSessionError {
    #[error(transparent)]
    ValidateSessionError(#[from] ValidateSessionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SessionService {
    pub(crate) async fn update(
        self,
        user_id: i32,
        metadata: String,
        refresh_token_to_validate: &str,
        new_refresh_token: String,
    ) -> Result<Session, UpdateSessionError> {
        let old_session = self
            .validate(user_id, &metadata, refresh_token_to_validate)
            .await?;

        let new_session = Session {
            refresh_token: new_refresh_token,
            ..old_session
        };

        let session =
            self.repo.update(new_session).await?.context(
                "the session existence was checked before updating it, but an error occurs",
            )?;

        Ok(session)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SaveSessionError {
    #[error("the limit on the sessions number has been reached, the maximum number of sessions is {}", .limit)]
    SessionsLimitReached { limit: usize },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SessionService {
    pub(crate) async fn save(
        self,
        user_id: i32,
        metadata: String,
        refresh_token: String,
    ) -> Result<Session, SaveSessionError> {
        let SessionsMaxNumber(sessions_max_number) = self.sessions_max_number;
        let SessionTTL(session_ttl) = self.session_ttl;

        let session = Session::new(user_id, metadata, refresh_token, session_ttl)?;

        let session = if self.is_session_exist(&session).await? {
            self.repo.update(session).await?.context(
                "the session existence was checked before updating it, but an error occurs",
            )?
        } else {
            self.check_user_limit(session.user_id, sessions_max_number)
                .await?;

            self.repo.insert(session).await?.context(
                "the session not existence was checked before inserting it, but an error occurs",
            )?
        };

        Ok(session)
    }

    async fn check_user_limit(
        &self,
        user_id: i32,
        sessions_max_number: usize,
    ) -> Result<(), SaveSessionError> {
        let sessions_number = self.repo.count_not_expired_by_user_id(user_id).await?;

        if sessions_number >= sessions_max_number {
            Err(SaveSessionError::SessionsLimitReached {
                limit: sessions_max_number,
            })
        } else {
            Ok(())
        }
    }
}
