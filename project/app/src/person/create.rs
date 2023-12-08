use crate::{base_repo::BaseRepoException, Outcome};

use super::{Entity, Service};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error(transparent)]
    Repo(#[from] BaseRepoException<Entity>),
}

impl Service {
    pub async fn create(&mut self, entity: Entity) -> Outcome<Entity, Exception> {
        let value = self.repo.insert(entity).await?;
        Outcome::Success(value)
    }
}

