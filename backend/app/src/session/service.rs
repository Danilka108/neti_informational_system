use anyhow::Context;

use crate::Outcome;

use super::{DynSessionRepository, Session, SessionTTL, SessionsMaxNumber};

pub struct SessionService {
    pub(crate) repo: DynSessionRepository,
    pub(crate) session_ttl: SessionTTL,
    pub(crate) sessions_max_number: SessionsMaxNumber,
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateSessionException {
    #[error("no session found")]
    NoSessionFound,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("session expired")]
    SessionExpired,
}

impl SessionService {
    async fn validate(
        &self,
        user_id: i32,
        metadata: &str,
        refresh_token: &str,
    ) -> Outcome<Session, ValidateSessionException> {
        let Some(session) = self.repo.find(user_id, metadata).await? else {
            self.repo.delete_all(user_id).await?;
            return Outcome::Exception(ValidateSessionException::NoSessionFound);
        };

        if session.refresh_token != refresh_token {
            self.repo.delete_all(user_id).await?;
            return Outcome::Exception(ValidateSessionException::InvalidRefreshToken);
        }

        if session.is_expired()? {
            self.repo.delete_all(user_id).await?;
            return Outcome::Exception(ValidateSessionException::SessionExpired);
        }

        Outcome::Success(session)
    }

    async fn is_session_exist(&self, session: &Session) -> Result<bool, anyhow::Error> {
        let result = self
            .repo
            .find(*session.user_id, &session.metadata)
            .await?
            .is_some();

        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSessionException {
    #[error(transparent)]
    FailedToValidateSession(#[from] ValidateSessionException),
}

impl SessionService {
    pub(crate) async fn delete(
        self,
        user_id: i32,
        metadata: &str,
        refresh_token_to_validate: &str,
    ) -> Outcome<Session, DeleteSessionException> {
        let _ = self
            .validate(user_id, metadata, refresh_token_to_validate)
            .await?;

        let deleted_session =
            self.repo.delete(user_id, metadata).await?.context(
                "the session existence was checked before deleting it, but an error occurs",
            )?;

        Outcome::Success(deleted_session)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateSessionException {
    #[error(transparent)]
    FailedToValidateSession(#[from] ValidateSessionException),
}

impl SessionService {
    pub(crate) async fn update(
        self,
        user_id: i32,
        metadata: String,
        refresh_token_to_validate: &str,
        new_refresh_token: String,
    ) -> Outcome<Session, UpdateSessionException> {
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

        Outcome::Success(session)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SaveSessionException {
    #[error("the limit on the sessions number has been reached, the maximum number of sessions is {}", .limit)]
    SessionsLimitReached { limit: usize },
}

impl SessionService {
    pub(crate) async fn save(
        self,
        user_id: i32,
        metadata: String,
        refresh_token: String,
    ) -> Outcome<Session, SaveSessionException> {
        let SessionsMaxNumber(sessions_max_number) = self.sessions_max_number;
        let SessionTTL(session_ttl) = self.session_ttl;

        let session = Session::new(user_id, metadata, refresh_token, session_ttl)?;

        let session = if self.is_session_exist(&session).await? {
            self.repo.update(session).await?.context(
                "the session existence was checked before updating it, but an error occurs",
            )?
        } else {
            self.check_user_limit(*session.user_id, sessions_max_number)
                .await?;

            self.repo.insert(session).await?.context(
                "the session not existence was checked before inserting it, but an error occurs",
            )?
        };

        Outcome::Success(session)
    }

    async fn check_user_limit(
        &self,
        user_id: i32,
        sessions_max_number: usize,
    ) -> Outcome<(), SaveSessionException> {
        let sessions_number = self.repo.count_not_expired_by_user_id(user_id).await?;

        if sessions_number >= sessions_max_number {
            Outcome::Exception(SaveSessionException::SessionsLimitReached {
                limit: sessions_max_number,
            })
        } else {
            Outcome::Success(())
        }
    }
}
