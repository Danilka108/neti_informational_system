use std::sync::Arc;

use app::api::{PasswordEncoder, SessionRepository, TokenManager, UserRepository};
use app::auth::AuthService;
use app::dyn_dependency;
use app::session::SessionService;
use app::token::TokenService;
use app::user::UserService;

use crate::env_config::EnvConfig;
use crate::pg::{init_pg_conn_pool, PgSessionRepository, PgTransaction, PgUserRepository};
use crate::security::password_encoder::Pbkdf2PassEncoder;
use crate::security::token_manager::TokenManagerImpl;

pub struct AppState {
    env_config: EnvConfig,
    pg_pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateError {
    #[error("failed to init postgres connection pool")]
    InitPgConnPoolError(#[from] sqlx::Error),
}

impl AppState {
    pub async fn new(env_config: EnvConfig) -> Result<Self, AppStateError> {
        let pg_pool = init_pg_conn_pool(&env_config).await?;

        // let token_manager = Arc::new();
        //
        // let user_repository = Arc::new(PgUserRepository);
        // let session_repository = Arc::new();

        Ok(Self {
            env_config,
            pg_pool,
        })
    }

    pub fn pg_pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.pg_pool
    }

    pub fn env_config(&self) -> &EnvConfig {
        &self.env_config
    }

    pub fn password_encoder(&self) -> dyn_dependency!(PasswordEncoder) {
        Box::new(Pbkdf2PassEncoder::new(self.env_config()))
    }

    pub fn token_manager(&self) -> dyn_dependency!(TokenManager) {
        Box::new(TokenManagerImpl::new(self.env_config()))
    }

    pub fn session_repository(
        &self,
    ) -> dyn_dependency!(SessionRepository<Transaction = PgTransaction>) {
        Box::new(PgSessionRepository)
    }

    pub fn user_repository(&self) -> dyn_dependency!(UserRepository<Transaction = PgTransaction>) {
        Box::new(PgUserRepository)
    }

    pub fn token_service(&self) -> TokenService {
        TokenService::new(
            self.env_config.jwt_token_ttl_in_seconds.into(),
            self.token_manager(),
        )
    }

    pub fn session_service(&self) -> SessionService<PgTransaction> {
        SessionService::new(
            self.env_config.sessions_max_number_per_user,
            self.env_config.session_ttl_in_seconds.into(),
            self.session_repository(),
        )
    }

    pub fn user_service(&self) -> UserService<PgTransaction> {
        UserService::new(self.user_repository(), self.password_encoder())
    }

    pub fn auth_service(&self) -> AuthService<PgTransaction> {
        AuthService::new(
            self.session_service(),
            self.user_service(),
            self.token_service(),
        )
    }
}
