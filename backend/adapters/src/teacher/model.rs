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
pub struct ClassTeachers {
    pub teacher_id: i32,
    pub study_group_id: i32,
    pub class_id: i32,
}

#[derive(Clone, Debug, FromRow)]
pub struct JoinRow {
    #[sqlx(flatten)]
    pub teacher: Teachers,
    #[sqlx(flatten)]
    pub class: ClassTeachers,
}

impl Teachers {
    pub fn into_entity(self, classes: Vec<ClassTeachers>) -> teacher::Entity {
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
// TODO
pub enum PgTeacherKind {
    assistant,
    regular_teacher,
    senior_teacher,
    associate_professor,
    professor,
}

impl From<TeacherKind> for PgTeacherKind {
    fn from(value: TeacherKind) -> Self {
        match value {
            TeacherKind::Assistant => PgTeacherKind::assistant,
            TeacherKind::RegularTeacher => PgTeacherKind::regular_teacher,
            TeacherKind::SeniorTeacher => PgTeacherKind::senior_teacher,
            TeacherKind::AssociateProfessor => PgTeacherKind::associate_professor,
            TeacherKind::Professor => PgTeacherKind::professor,
        }
    }
}

impl From<PgTeacherKind> for TeacherKind {
    fn from(value: PgTeacherKind) -> Self {
        match value {
            PgTeacherKind::assistant => TeacherKind::Assistant,
            PgTeacherKind::regular_teacher => TeacherKind::RegularTeacher,
            PgTeacherKind::senior_teacher => TeacherKind::SeniorTeacher,
            PgTeacherKind::associate_professor => TeacherKind::AssociateProfessor,
            PgTeacherKind::professor => TeacherKind::Professor,
        }
    }
}
impl Display for PgTeacherKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PgTeacherKind::assistant => "assistant",
                PgTeacherKind::regular_teacher => "regular_teacher",
                PgTeacherKind::senior_teacher => "senior_teacher",
                PgTeacherKind::associate_professor => "associate_professor",
                PgTeacherKind::professor => "professor",
            }
        )
    }
}
