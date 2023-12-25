use anyhow::Context;
use std::sync::Arc;
use tokio::sync::Mutex;
use utils::di::{Module, Provide};

use crate::{
    access_token::JwtAccessTokenEngine, attestation::PgAttestationRepo, class::PgClassRepo,
    class_kind::PgClassKindRepo, config::ConfigModule, curriculum::PgCurriculumRepo,
    curriculum_module::PgCurriculumModuleRepo, discipline::PgDisciplineRepo,
    hasher::Argon2PasswordHasher, passport::PgPassportRepo, person::PgPersonRepo,
    refresh_token::NanoIdRefreshTokenGenerator, student::PgStudentRepo,
    study_group::PgStudyGroupRepo, subdivision::PgSubdivisionRepo, tag::PgTagRepo,
    teacher::PgTeacherRepo, university::PgUniversityRepo, user::PgUserRepo,
    user_session::PgUserSessionRepo, PgTransaction,
};

#[derive(Debug, Clone)]
pub struct TransactionModule<C: ConfigModule> {
    pub config: C,
    pub(crate) txn: Arc<Mutex<PgTransaction<'static>>>,
}

impl<C: ConfigModule> TransactionModule<C> {
    pub async fn commit(self) -> Result<(), anyhow::Error> {
        let txn =
            Arc::into_inner(self.txn).context("transaction has more than 1 strong reference")?;
        Mutex::into_inner(txn).commit().await?;

        Ok(())
    }
}

impl<C: ConfigModule> Module for TransactionModule<C> {}

impl<C: ConfigModule + Send> app::AdaptersModule for TransactionModule<C> {}

impl<C: ConfigModule> Provide<app::attestation::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::attestation::BoxedRepo {
        Box::new(PgAttestationRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::class::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::class::BoxedRepo {
        Box::new(PgClassRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::class_kind::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::class_kind::BoxedRepo {
        Box::new(PgClassKindRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::curriculum::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::curriculum::BoxedRepo {
        Box::new(PgCurriculumRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::curriculum_module::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::curriculum_module::BoxedRepo {
        Box::new(PgCurriculumModuleRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::discipline::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::discipline::BoxedRepo {
        Box::new(PgDisciplineRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::passport::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::passport::BoxedRepo {
        Box::new(PgPassportRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::person::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::person::BoxedRepo {
        Box::new(PgPersonRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::student::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::student::BoxedRepo {
        Box::new(PgStudentRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::study_group::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::study_group::BoxedRepo {
        Box::new(PgStudyGroupRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::subdivision::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::subdivision::BoxedRepo {
        Box::new(PgSubdivisionRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::tag::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::tag::BoxedRepo {
        Box::new(PgTagRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::teacher::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::teacher::BoxedRepo {
        Box::new(PgTeacherRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::university::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::university::BoxedRepo {
        Box::new(PgUniversityRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::user::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::user::BoxedRepo {
        Box::new(PgUserRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::user_session::BoxedRepo> for TransactionModule<C> {
    fn provide(&self) -> app::user_session::BoxedRepo {
        Box::new(PgUserSessionRepo {
            txn: Arc::clone(&self.txn),
        })
    }
}

impl<C: ConfigModule> Provide<app::hasher::BoxedPasswordHasher> for TransactionModule<C> {
    fn provide(&self) -> app::hasher::BoxedPasswordHasher {
        Box::new(Argon2PasswordHasher::new(self.config.resolve()))
    }
}

impl<C: ConfigModule> Provide<app::token::BoxedAccessTokenEngine> for TransactionModule<C> {
    fn provide(&self) -> app::token::BoxedAccessTokenEngine {
        Box::new(JwtAccessTokenEngine {
            keys: self.config.resolve(),
        })
    }
}

impl<C: ConfigModule> Provide<app::token::BoxedRefreshTokenGenerator> for TransactionModule<C> {
    fn provide(&self) -> app::token::BoxedRefreshTokenGenerator {
        Box::new(NanoIdRefreshTokenGenerator {
            length: self.config.resolve(),
        })
    }
}

impl<C: ConfigModule> Provide<app::token::AccessTokenTTL> for TransactionModule<C> {
    fn provide(&self) -> app::token::AccessTokenTTL {
        self.config.resolve()
    }
}

impl<C: ConfigModule> Provide<app::user_session::SessionTTL> for TransactionModule<C> {
    fn provide(&self) -> app::user_session::SessionTTL {
        self.config.resolve()
    }
}

impl<C: ConfigModule> Provide<app::user_session::SessionsMaxNumber> for TransactionModule<C> {
    fn provide(&self) -> app::user_session::SessionsMaxNumber {
        self.config.resolve()
    }
}
