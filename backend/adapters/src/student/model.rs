use app::student;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Students {
    pub id: i32,
    pub person_id: i32,
    pub study_group_id: i32,
}

impl Students {
    pub fn into_entity(self, attestations: Vec<StudentAttestations>) -> student::Entity {
        student::Entity {
            id: Id::new(self.id),
            person_id: Id::new(self.person_id),
            study_group_id: Id::new(self.study_group_id),
            attestations: attestations
                .into_iter()
                .map(|v| student::StudentAttestation {
                    attestation_id: Id::new(v.attestation_id),
                    score: v.score,
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct StudentAttestations {
    pub student_id: i32,
    pub attestation_id: i32,
    pub score: i32,
}

#[derive(Clone, Debug, FromRow)]
pub struct JoinRow {
    #[sqlx(flatten)]
    pub student: Students,
    #[sqlx(flatten)]
    pub attestation: StudentAttestations,
}
