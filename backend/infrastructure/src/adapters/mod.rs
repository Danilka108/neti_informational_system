mod access_token_engine;
pub mod config;
mod password_hasher;
mod person_repository;
mod refresh_token_generator;
mod session_repository;
mod user_repository;

use std::sync::Arc;

use crate::pg::PgTransaction;
use access_token_engine::{JwtAccessTokenEngine, JwtKeys};
use app::{
    ports::DynPersonRepository,
    session::{DynSessionRepository, SessionTTL, SessionsMaxNumber},
    token::{AccessTokenTTL, DynAccessTokenEngine, DynRefreshTokenGenerator},
    user::{DynPasswordHasher, DynUserRepository},
};
use di::{Module, Provide};
use password_hasher::{Argon2Params, Argon2PasswordHasher};
use person_repository::PgPersonRepository;
use refresh_token_generator::{NanoIdRefreshTokenGenerator, RefreshTokenLength};
use session_repository::PgSessionRepository;
use tokio::sync::Mutex;
use user_repository::PgUserRepository;

pub trait ConfigModule:
    Module
    + Provide<Arc<JwtKeys>>
    + Provide<RefreshTokenLength>
    + Provide<Argon2Params>
    + Provide<AccessTokenTTL>
    + Provide<SessionTTL>
    + Provide<SessionsMaxNumber>
{
}

#[derive(Debug, Clone)]
pub struct AdaptersModule<C> {
    pub config: C,
    pub txn: Arc<Mutex<PgTransaction>>,
}

impl<P> Module for AdaptersModule<P> {}

impl<P: ConfigModule> app::AdaptersModule for AdaptersModule<P> {}

impl<P: ConfigModule> Provide<DynPersonRepository> for AdaptersModule<P> {
    fn provide(&self) -> DynPersonRepository {
        Box::new(PgPersonRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<DynAccessTokenEngine> for AdaptersModule<P> {
    fn provide(&self) -> DynAccessTokenEngine {
        Box::new(JwtAccessTokenEngine {
            keys: self.config.resolve(),
        })
    }
}

impl<P: ConfigModule> Provide<DynRefreshTokenGenerator> for AdaptersModule<P> {
    fn provide(&self) -> DynRefreshTokenGenerator {
        Box::new(NanoIdRefreshTokenGenerator {
            length: self.config.resolve(),
        })
    }
}

impl<P: ConfigModule> Provide<DynPasswordHasher> for AdaptersModule<P> {
    fn provide(&self) -> DynPasswordHasher {
        Box::new(Argon2PasswordHasher::new(self.config.resolve()))
    }
}

impl<P: ConfigModule> Provide<DynSessionRepository> for AdaptersModule<P> {
    fn provide(&self) -> DynSessionRepository {
        Box::new(PgSessionRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<DynUserRepository> for AdaptersModule<P> {
    fn provide(&self) -> DynUserRepository {
        Box::new(PgUserRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<AccessTokenTTL> for AdaptersModule<P> {
    fn provide(&self) -> AccessTokenTTL {
        self.config.resolve()
    }
}

impl<P: ConfigModule> Provide<SessionTTL> for AdaptersModule<P> {
    fn provide(&self) -> SessionTTL {
        self.config.resolve()
    }
}

impl<P: ConfigModule> Provide<SessionsMaxNumber> for AdaptersModule<P> {
    fn provide(&self) -> SessionsMaxNumber {
        self.config.resolve()
    }
}
