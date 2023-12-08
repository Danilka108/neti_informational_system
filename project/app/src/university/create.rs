use crate::{base_repo::BaseRepoException, Outcome};

use super::{Entity, Id, Service};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error(transparent)]
    Repo(#[from] BaseRepoException<Entity>),
}

impl Service {
    pub async fn create(&mut self, name: String) -> Outcome<Entity, Exception> {
        let value = self
            .repo
            .insert(Entity {
                id: Id::new(0),
                name,
            })
            .await?;
        Outcome::Success(value)
    }
}
