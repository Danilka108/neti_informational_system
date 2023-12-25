use std::collections::HashSet;

use utils::entity::entity;

use crate::{attestation, person, study_group};

mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

#[derive(Clone, PartialEq, Eq)]
#[entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub person_id: person::EntityId,
    pub study_group_id: study_group::EntityId,
    pub attestations: HashSet<StudentAttestation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StudentAttestation {
    pub attestation_id: attestation::EntityId,
    pub score: i32,
}
