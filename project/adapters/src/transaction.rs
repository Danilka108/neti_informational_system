use anyhow::Context;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    adapters::{
        self, password,
        person::PgPersonRepository,
        subdivision::{
            PgSubdivisionMemberRepository, PgSubdivisionRepository, PgSubdivisionTagRepository,
        },
        tag::PgTagRepository,
        tokens::{JwtAccessTokenEngine, NanoIdRefreshTokenGenerator},
        university::PgUniveristyRepository,
    },
    pg::PgTransaction,
};
use app::ports::*;
use di::{Module, Provide};

pub use crate::config::ConfigModule;

#[derive(Debug, Clone)]
pub struct TransactionModule<C> {
    pub(crate) config: C,
    pub(crate) txn: Arc<Mutex<PgTransaction>>,
}

impl<C> TransactionModule<C> {
    pub async fn commit(self) -> Result<(), anyhow::Error> {
        let txn =
            Arc::into_inner(self.txn).context("transaction has more than 1 strong reference")?;
        Mutex::into_inner(txn).commit().await?;

        Ok(())
    }
}

impl<P> Module for TransactionModule<P> {}

impl<'c, P: ConfigModule> app::AdaptersModule for TransactionModule<P> {}

impl<P: ConfigModule> Provide<DynPersonRepository> for TransactionModule<P> {
    fn provide(&self) -> DynPersonRepository {
        Box::new(PgPersonRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<DynAccessTokenEngine> for TransactionModule<P> {
    fn provide(&self) -> DynAccessTokenEngine {
        Box::new(JwtAccessTokenEngine {
            keys: self.config.resolve(),
        })
    }
}

impl<P: ConfigModule> Provide<DynRefreshTokenGenerator> for TransactionModule<P> {
    fn provide(&self) -> DynRefreshTokenGenerator {
        Box::new(NanoIdRefreshTokenGenerator {
            length: self.config.resolve(),
        })
    }
}

impl<P: ConfigModule> Provide<DynPasswordHasher> for TransactionModule<P> {
    fn provide(&self) -> DynPasswordHasher {
        Box::new(password::Argon2PasswordHasher::new(self.config.resolve()))
    }
}

impl<P: ConfigModule> Provide<DynSessionRepository> for TransactionModule<P> {
    fn provide(&self) -> DynSessionRepository {
        Box::new(adapters::session::PgSessionRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<app::user::DynUserRepository> for TransactionModule<P> {
    fn provide(&self) -> app::ports::DynUserRepository {
        Box::new(adapters::user::PgUserRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<AccessTokenTTL> for TransactionModule<P> {
    fn provide(&self) -> AccessTokenTTL {
        self.config.resolve()
    }
}

impl<P: ConfigModule> Provide<SessionTTL> for TransactionModule<P> {
    fn provide(&self) -> SessionTTL {
        self.config.resolve()
    }
}

impl<P: ConfigModule> Provide<SessionsMaxNumber> for TransactionModule<P> {
    fn provide(&self) -> SessionsMaxNumber {
        self.config.resolve()
    }
}

impl<P: ConfigModule> Provide<BoxedUniversityRepository> for TransactionModule<P> {
    fn provide(&self) -> BoxedUniversityRepository {
        Box::new(PgUniveristyRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<BoxedSubdivisionRepository> for TransactionModule<P> {
    fn provide(&self) -> BoxedSubdivisionRepository {
        Box::new(PgSubdivisionRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<BoxedSubdivisionMemberRepository> for TransactionModule<P> {
    fn provide(&self) -> BoxedSubdivisionMemberRepository {
        Box::new(PgSubdivisionMemberRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<BoxedSubdivisionTagRepository> for TransactionModule<P> {
    fn provide(&self) -> BoxedSubdivisionTagRepository {
        Box::new(PgSubdivisionTagRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<P: ConfigModule> Provide<BoxedTagRepository> for TransactionModule<P> {
    fn provide(&self) -> BoxedTagRepository {
        Box::new(PgTagRepository {
            txn: Arc::clone(&self.txn),
        })
    }
}
