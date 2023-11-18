use std::num::NonZeroI32;

use crate::{ports::EntityDoesNotExistError, subdivision::Subdivision, NonIdentified};

use super::University;

#[async_trait::async_trait]
pub trait UniversityRepository {
    async fn save(
        &self,
        university: University<NonIdentified>,
    ) -> Result<University, anyhow::Error>;

    async fn delete(
        &self,
        id: NonZeroI32,
    ) -> Result<Result<University, EntityDoesNotExistError>, anyhow::Error>;

    async fn add_subdivisions(
        &self,
        id: NonZeroI32,
        subdivisions: Vec<Subdivision>,
    ) -> Result<Result<University, EntityDoesNotExistError>, anyhow::Error>;

    async fn remove_subdivisions(
        &self,
        id: NonZeroI32,
        subdivisions: Vec<Subdivision>,
    ) -> Result<Result<University, EntityDoesNotExistError>, anyhow::Error>;

    async fn get(
        &self,
        id: NonZeroI32,
    ) -> Result<Result<University, EntityDoesNotExistError>, anyhow::Error>;
}
