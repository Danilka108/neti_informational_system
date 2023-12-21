use utils::entity::entity;

use crate::{attestation, person, study_group};

#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub person_id: person::EntityId,
    pub study_group_id: study_group::EntityId,
    pub attestations: Box<[StudentAttestation]>,
}

pub struct StudentAttestation {
    pub attestation_id: attestation::EntityId,
    pub score: u8,
    pub raiting_contributor_id: person::EntityId,
}
