use crate::{base_repo::BaseRepoException, university, Outcome};

use super::{Entity, Id, Service};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("invalid university id, {}", .0)]
    UniversityNotFound(university::get::Exception),
    #[error(transparent)]
    Repo(#[from] BaseRepoException<Entity>),
}

impl Service {
    pub async fn create(
        &mut self,
        name: String,
        university_id: university::Id,
    ) -> Outcome<Entity, Exception> {
        let _university = self
            .university_service
            .get(university_id)
            .await
            .map_exception(Exception::UniversityNotFound)?;

        let entity = Entity {
            id: Id::new(0),
            name,
            university_id,
        };
        let value = self.repo.insert(entity).await?;

        Outcome::Success(value)
    }
}
