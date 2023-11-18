use std::num::NonZeroI32;

use crate::{NonIdentified, Outcome};

use super::{exceptions::*, BoxedUniversityRepository, University};

pub struct UniversityService {
    pub(crate) repo: BoxedUniversityRepository,
}

impl UniversityService {
    pub async fn create(self, name: String) -> Outcome<University, CreateUniversityException> {
        let university = self
            .repo
            .save(University {
                id: NonIdentified,
                name,
            })
            .await?;

        Outcome::Success(university)
    }

    pub async fn delete(self, id: NonZeroI32) -> Outcome<University, DeleteUniversityException> {
        let Ok(university) = self.repo.delete(id).await? else {
            return Outcome::Exception(DeleteUniversityException::UniversityDoesNotExist(
                UniversityDoesNotExistError { id },
            ));
        };

        Outcome::Success(university)
    }

    pub async fn get(self, id: NonZeroI32) -> Outcome<University, UniversityDoesNotExistError> {
        let Ok(university) = self.repo.get(id).await? else {
            return Outcome::Exception(UniversityDoesNotExistError { id });
        };

        Outcome::Success(university)
    }
}
