use crate::{
    api::{EntityDoesNotExistError, SessionRepository},
    dyn_dependency,
};
use anyhow::Context;
use domain::{Seconds, SecondsFromUnixEpoch, Session};

pub struct SessionService<T> {
    repo: dyn_dependency!(SessionRepository<Transaction = T>),
    sessions_max_number: usize,
    session_ttl: Seconds,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSessionError {
    #[error("session does not exist")]
    SessionDoesNotExist(#[from] EntityDoesNotExistError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SaveSessionError {
    #[error("the limit on the sessions number has been reached, the maximum number of sessions is {}", .limit)]
    SessionsLimitReached { limit: usize },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateSessionError {
    #[error("no session found")]
    NoSessionFound,
    #[error("invlid refresh token")]
    InvalidRefreshToken,
    #[error("session expired")]
    SessionExpired,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl<T> SessionService<T> {
    pub fn new(
        sessions_max_number: usize,
        session_ttl: Seconds,
        repo: dyn_dependency!(SessionRepository<Transaction = T>),
    ) -> Self {
        Self {
            repo,
            session_ttl,
            sessions_max_number,
        }
    }

    pub(super) async fn delete_session(
        &self,
        tx: &mut T,
        session: Session,
    ) -> Result<Session, DeleteSessionError> {
        Ok(self.repo.delete(tx, session).await??)
    }

    pub(super) async fn validate_refresh_token(
        &self,
        tx: &mut T,
        user_id: i32,
        metadata: &str,
        refresh_token_to_validate: &str,
    ) -> Result<Session, ValidateSessionError> {
        let Some(session) = self.repo.find(tx, user_id, metadata).await? else {
            return Err(ValidateSessionError::NoSessionFound);
        };

        if session.refresh_token != refresh_token_to_validate {
            return Err(ValidateSessionError::InvalidRefreshToken);
        }

        if session.expires_at.is_expired()? {
            return Err(ValidateSessionError::SessionExpired);
        }

        Ok(session)
    }

    pub(super) async fn save_session(
        &self,
        tx: &mut T,
        user_id: i32,
        metadata: String,
        refresh_token: String,
    ) -> Result<Session, SaveSessionError> {
        let expires_at = SecondsFromUnixEpoch::new_expires_at(self.session_ttl)
            .context("failed to generate new expires at")?;

        let session = Session {
            user_id,
            metadata,
            refresh_token,
            expires_at,
        };

        if self.is_session_already_present(tx, &session).await? {
            self.repo
                .update(tx, session.clone())
                .await
                .context("failed to update session repository")?
                .context("session existence is checked before updating it, but an error occurs")?;
        } else {
            self.check_user_limit(tx, session.user_id).await?;
            self.repo
                .insert(tx, session.clone())
                .await
                .context("failed to insert into session repository")?
                .context(
                    "session not existence is checked before inserting it, but an error occurs",
                )?;
        }

        Ok(session)
    }

    async fn is_session_already_present(
        &self,
        tx: &mut T,
        session: &Session,
    ) -> Result<bool, anyhow::Error> {
        let result = self
            .repo
            .find(tx, session.user_id, &session.metadata)
            .await
            .context("failed to read session repository")?
            .is_some();

        Ok(result)
    }

    async fn check_user_limit(&self, tx: &mut T, user_id: i32) -> Result<(), SaveSessionError> {
        let sessions_number = self
            .repo
            .count_not_expired_by_user_id(tx, user_id)
            .await
            .context("failed to read session repository")?;

        if sessions_number >= self.sessions_max_number {
            Err(SaveSessionError::SessionsLimitReached {
                limit: self.sessions_max_number,
            })
        } else {
            Ok(())
        }
    }
}
