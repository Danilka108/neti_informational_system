use std::fmt::Display;

use app::teacher::{self, TeacherKind};
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Teachers {
    pub id: i32,
    pub person_id: i32,
    pub kind: PgTeacherKind,
    pub department_id: i32,
}

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct TeacherClasses {
    pub teacher_id: i32,
    pub study_group_id: i32,
    pub class_id: i32,
}

#[derive(Clone, Debug, FromRow)]
pub struct JoinRow {
    #[sqlx(flatten)]
    pub teacher: Teachers,
    #[sqlx(flatten)]
    pub class: TeacherClasses,
}

impl Teachers {
    pub fn into_entity(self, classes: Vec<TeacherClasses>) -> teacher::Entity {
        teacher::Entity {
            id: Id::new(self.id),
            person_id: Id::new(self.person_id),
            kind: self.kind.into(),
            department_id: Id::new(self.department_id),
            classes: classes
                .into_iter()
                .map(|v| teacher::TeacherClass {
                    study_group_id: Id::new(v.study_group_id),
                    class_id: Id::new(v.class_id),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "teacher_kind")]
#[sqlx(rename_all = "lowercase")]
pub enum PgTeacherKind {
    Assistant,
    RegularTeacher,
    SeniorTeacher,
    AssociateProfessor,
    Professor,
}

impl From<TeacherKind> for PgTeacherKind {
    fn from(value: TeacherKind) -> Self {
        match value {
            TeacherKind::Assistant => PgTeacherKind::Assistant,
            TeacherKind::RegularTeacher => PgTeacherKind::RegularTeacher,
            TeacherKind::SeniorTeacher => PgTeacherKind::SeniorTeacher,
            TeacherKind::AssociateProfessor => PgTeacherKind::AssociateProfessor,
            TeacherKind::Professor => PgTeacherKind::Professor,
        }
    }
}

impl From<PgTeacherKind> for TeacherKind {
    fn from(value: PgTeacherKind) -> Self {
        match value {
            PgTeacherKind::Assistant => TeacherKind::Assistant,
            PgTeacherKind::RegularTeacher => TeacherKind::RegularTeacher,
            PgTeacherKind::SeniorTeacher => TeacherKind::SeniorTeacher,
            PgTeacherKind::AssociateProfessor => TeacherKind::AssociateProfessor,
            PgTeacherKind::Professor => TeacherKind::Professor,
        }
    }
}
impl Display for PgTeacherKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PgTeacherKind::Assistant => "assistant",
                PgTeacherKind::RegularTeacher => "regular_teacher",
                PgTeacherKind::SeniorTeacher => "senior_teacher",
                PgTeacherKind::AssociateProfessor => "associate_professor",
                PgTeacherKind::Professor => "professor",
            }
        )
    }
}
