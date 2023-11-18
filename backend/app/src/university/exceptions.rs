use std::num::NonZeroI32;

#[derive(Debug, thiserror::Error)]
#[error("university {id} does not exist")]
pub struct UniversityDoesNotExistError {
    pub(crate) id: NonZeroI32,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUniversityException {}

#[derive(Debug, thiserror::Error)]
pub enum DeleteUniversityException {
    #[error(transparent)]
    UniversityDoesNotExist(#[from] UniversityDoesNotExistError),
}

#[derive(Debug, thiserror::Error)]
pub enum AddUniversitySubdivisionException {
    #[error(transparent)]
    UniversityDoesNotExist(#[from] UniversityDoesNotExistError),
}
