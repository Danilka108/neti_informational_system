use crate::{base_repo::BaseRepoException, university, Outcome};

use super::{Entity, Service};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("invalid university id, {}", .0)]
    UniversityNotFound(university::get::Exception),
    #[error(transparent)]
    Repo(#[from] BaseRepoException<Entity>),
}

impl Service {
    pub async fn get_by_univeristy(
        &self,
        university_id: university::Id,
    ) -> Outcome<Vec<Entity>, Exception> {
        let _university = self
            .university_service
            .get(university_id)
            .await
            .map_exception(Exception::UniversityNotFound)?;

        let values = self.repo.list_by_university(university_id).await?;
        Outcome::Success(values)
    }
}
