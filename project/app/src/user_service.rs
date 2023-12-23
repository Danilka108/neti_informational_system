use utils::{di::Provide, entity::Id, outcome::Outcome};

use crate::{
    hasher, token::BoxedRefreshTokenGenerator, user, user_session, AdaptersModule, AppModule,
};

pub struct UserService {
    repo: user::BoxedRepo,
    session_repo: user_session::BoxedRepo,
    hasher: hasher::BoxedPasswordHasher,
    refresh_token_generator: BoxedRefreshTokenGenerator,
    session_ttl: user_session::SessionTTL,
    sessions_max_number: user_session::SessionsMaxNumber,
}

impl<A: AdaptersModule> Provide<UserService> for AppModule<A> {
    fn provide(&self) -> UserService {
        UserService {
            repo: self.adapters.resolve(),
            session_repo: self.adapters.resolve(),
            hasher: self.adapters.resolve(),
            refresh_token_generator: self.adapters.resolve(),
            session_ttl: self.adapters.resolve(),
            sessions_max_number: self.adapters.resolve(),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UserException {
    #[error("email already in use")]
    EmailAlreadyInUse,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid password")]
    InvalidPassword,
    #[error("session not found")]
    SessionNotFound,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("session expired")]
    SessionExpired,
    #[error("sessions limit reached")]
    SessionsLimitReached,
}

impl UserService {
    pub async fn create(
        &mut self,
        email: String,
        password: String,
    ) -> Outcome<user::Entity, UserException> {
        if self.repo.find_by_email(email.clone()).await?.is_some() {
            return Outcome::Ex(UserException::EmailAlreadyInUse);
        }

        let user = user::Entity {
            id: Default::default(),
            email,
            password: self.hasher.hash(password).await?,
        };

        let user = self.repo.save(user).await?;
        Outcome::Ok(user)
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Outcome<user::Entity, UserException> {
        let Some(user) = self.repo.find_by_email(email).await? else {
            return Outcome::Ex(UserException::UserNotFound);
        };

        let hashed_password = self.hasher.hash(password).await?;
        if user.password != hashed_password {
            return Outcome::Ex(UserException::InvalidPassword);
        }

        Outcome::Ok(user)
    }

    pub async fn get(&self, user_id: user::EntityId) -> Outcome<user::Entity, UserException> {
        let Some(user) = self.repo.find(user_id).await? else {
            return Outcome::Ex(UserException::UserNotFound);
        };

        Outcome::Ok(user)
    }

    pub async fn create_session(
        &mut self,
        user_id: user::EntityId,
        metadata: String,
    ) -> Outcome<user_session::Entity, UserException> {
        let user_session::SessionTTL(ttl) = self.session_ttl;

        let id = Id::new(user_session::Id { user_id, metadata });
        let refresh_token = self.refresh_token_generator.generate().await?;
        let expires_at = user_session::SecondsFromUnixEpoch::expired_at_from_ttl(ttl)?;

        let session = user_session::Entity {
            id: id.clone(),
            refresh_token,
            expires_at,
        };

        if self.session_repo.find(id).await?.is_none() {
            self.check_limit(user_id).await?;
        }

        let session = self.session_repo.save(session).await?;
        Outcome::Ok(session)
    }

    pub async fn update_session(
        &mut self,
        user_id: user::EntityId,
        metadata: String,
        refresh_token_to_validate: String,
    ) -> Outcome<user_session::Entity, UserException> {
        let old_session = self
            .get_validated_session(user_id, metadata, refresh_token_to_validate)
            .await?;

        let new_refresh_token = self.refresh_token_generator.generate().await?;

        let session = user_session::Entity {
            refresh_token: new_refresh_token,
            ..old_session
        };

        let session = self.session_repo.save(session).await?;
        Outcome::Ok(session)
    }

    pub async fn remove_session(
        &mut self,
        user_id: user::EntityId,
        metadata: String,
        refresh_token_to_validate: String,
    ) -> Outcome<user_session::Entity, UserException> {
        let session = self
            .get_validated_session(user_id, metadata, refresh_token_to_validate)
            .await?;

        self.session_repo.delete(&session).await?;
        Outcome::Ok(session)
    }

    async fn check_limit(&self, user_id: user::EntityId) -> Outcome<(), UserException> {
        let user_session::SessionsMaxNumber(max_number) = self.sessions_max_number;

        let current_number = self.session_repo.count_not_expired(user_id).await?;
        if current_number + 1 > max_number {
            return Outcome::Ex(UserException::SessionsLimitReached);
        }

        Outcome::Ok(())
    }

    async fn get_validated_session(
        &self,
        user_id: user::EntityId,
        metadata: String,
        refresh_token_to_validate: String,
    ) -> Outcome<user_session::Entity, UserException> {
        let id = Id::new(user_session::Id { user_id, metadata });

        let Some(session) = self.session_repo.find(id).await? else {
            return Outcome::Ex(UserException::SessionNotFound);
        };

        if session.refresh_token != refresh_token_to_validate {
            return Outcome::Ex(UserException::InvalidRefreshToken);
        }

        if session.expires_at.is_expired()? {
            return Outcome::Ex(UserException::SessionExpired);
        }

        Outcome::Ok(session)
    }
}
