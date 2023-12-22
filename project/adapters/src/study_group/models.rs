use std::fmt::Display;

use app::study_group::{self, Qualification, TrainingKind};
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct StudyGroups {
    pub id: i32,
    pub name: String,
    pub studying_qualification: PgQualification,
    pub training_kind: PgTrainingKind,
    pub department_id: i32,
}

#[derive(Clone, Debug, FromRow)]
pub struct JoinRow {
    #[sqlx(flatten)]
    pub study_group: StudyGroups,
    #[sqlx(flatten)]
    pub curriculum: StudyGroupCurriculums,
}

impl StudyGroups {
    pub fn into_entity(self, curriculums: Vec<StudyGroupCurriculums>) -> study_group::Entity {
        study_group::Entity {
            id: Id::new(self.id),
            name: self.name,
            studying_qualification: self.studying_qualification.into(),
            training_kind: self.training_kind.into(),
            department_id: Id::new(self.department_id),
            curriculums: curriculums
                .into_iter()
                .map(|v| Id::new(v.curriculum_id))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct StudyGroupCurriculums {
    pub study_group_id: i32,
    pub curriculum_id: i32,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "qualification")]
#[sqlx(rename_all = "lowercase")]
pub enum PgQualification {
    Bachelor,
    Master,
    Postgraduate,
    Doctorate,
}

impl Display for PgQualification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PgQualification::Doctorate => "doctorate",
                PgQualification::Master => "master",
                PgQualification::Postgraduate => "postgraduate",
                PgQualification::Bachelor => "bachelor",
            }
        )
    }
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "training_kind")]
#[sqlx(rename_all = "lowercase")]
pub enum PgTrainingKind {
    FullTime,
    Correspondence,
}
impl Display for PgTrainingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PgTrainingKind::FullTime => "full_time",
                PgTrainingKind::Correspondence => "correspondence",
            }
        )
    }
}

impl From<Qualification> for PgQualification {
    fn from(value: Qualification) -> Self {
        match value {
            Qualification::Doctorate => PgQualification::Doctorate,
            Qualification::Master => PgQualification::Master,
            Qualification::Postgraduate => PgQualification::Postgraduate,
            Qualification::Bachelor => PgQualification::Bachelor,
        }
    }
}

impl From<TrainingKind> for PgTrainingKind {
    fn from(value: TrainingKind) -> Self {
        match value {
            TrainingKind::FullTime => PgTrainingKind::FullTime,
            TrainingKind::Correspondence => PgTrainingKind::Correspondence,
        }
    }
}

impl From<PgQualification> for Qualification {
    fn from(value: PgQualification) -> Self {
        match value {
            PgQualification::Doctorate => Qualification::Doctorate,
            PgQualification::Master => Qualification::Master,
            PgQualification::Postgraduate => Qualification::Postgraduate,
            PgQualification::Bachelor => Qualification::Bachelor,
        }
    }
}

impl From<PgTrainingKind> for TrainingKind {
    fn from(value: PgTrainingKind) -> Self {
        match value {
            PgTrainingKind::FullTime => TrainingKind::FullTime,
            PgTrainingKind::Correspondence => TrainingKind::Correspondence,
        }
    }
}
