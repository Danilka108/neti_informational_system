use crate::{base_repo::BaseRepoException, Outcome};

use super::{Entity, Id, Service};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error(transparent)]
    Repo(#[from] BaseRepoException<Entity>),
}

impl Service {
    pub async fn get(&self, id: Id) -> Outcome<Entity, Exception> {
        let value = self.repo.find_by_id(id).await?;

        Outcome::Success(value)
    }
}
