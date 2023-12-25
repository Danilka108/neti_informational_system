use std::fmt::Display;

use app::attestation::{self, AttestationKind};
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Attestations {
    pub id: i32,
    pub curriculum_module_id: i32,
    pub kind: PgAttestationKind,
    pub duration: i32,
}

impl Attestations {
    pub fn into_entity(self, examiners: Vec<AttestationExaminers>) -> attestation::Entity {
        attestation::Entity {
            id: Id::new(self.id),
            curriculum_module_id: Id::new(self.curriculum_module_id),
            kind: self.kind.into(),
            duration: attestation::Hours(self.duration),
            examiners: examiners
                .into_iter()
                .map(|v| v.examiner_id)
                .map(Id::new)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct AttestationExaminers {
    pub attestation_id: i32,
    pub examiner_id: i32,
}

#[derive(Clone, Debug, FromRow)]
pub struct JoinRow {
    #[sqlx(flatten)]
    pub attestation: Attestations,
    #[sqlx(flatten)]
    pub examiner: AttestationExaminers,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "attestation_kind")]
#[sqlx(rename_all = "lowercase")]
pub enum PgAttestationKind {
    Test,
    DiffTest,
    Exam,
}

impl From<AttestationKind> for PgAttestationKind {
    fn from(value: AttestationKind) -> Self {
        match value {
            AttestationKind::Test => PgAttestationKind::Test,
            AttestationKind::Exam => PgAttestationKind::Exam,
            AttestationKind::DiffTest => PgAttestationKind::DiffTest,
        }
    }
}

impl From<PgAttestationKind> for AttestationKind {
    fn from(value: PgAttestationKind) -> Self {
        match value {
            PgAttestationKind::Test => AttestationKind::Test,
            PgAttestationKind::Exam => AttestationKind::Exam,
            PgAttestationKind::DiffTest => AttestationKind::DiffTest,
        }
    }
}

impl Display for PgAttestationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Test => "test",
                Self::DiffTest => "diff_test",
                Self::Exam => "exam",
            }
        )
    }
}
