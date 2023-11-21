use std::num::NonZeroI32;

use crate::{
    ports::{EntityAlreadyExistError, EntityDoesNotExistError, EntityNotFoundError},
    Outcome,
};

use super::{exceptions::*, BoxedUniversityRepository, University};

pub struct UniversityService {
    pub(crate) repo: BoxedUniversityRepository,
}

impl UniversityService {
    pub async fn create(self, name: String) -> Outcome<University, CreateUniversityException> {
        self.repo
            .insert(University {
                id: (),
                name: name.clone(),
            })
            .await
            .map_exception(|EntityAlreadyExistError| {
                CreateUniversityException::UniversityAlreadyExists { name }
            })
    }

    pub async fn delete(self, id: NonZeroI32) -> Outcome<University, DeleteUniversityException> {
        self.repo
            .delete(id)
            .await
            .map_exception(|EntityDoesNotExistError| {
                DeleteUniversityException::UniversityDoesNotExist(UniversityDoesNotExistError {
                    id,
                })
            })
    }

    pub async fn get(self, id: NonZeroI32) -> Outcome<University, UniversityDoesNotExistError> {
        self.repo
            .get(id)
            .await
            .map_exception(|EntityNotFoundError| UniversityDoesNotExistError { id })
    }
}
