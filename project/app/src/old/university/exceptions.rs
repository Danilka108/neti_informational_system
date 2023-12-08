use crate::SerialId;

#[derive(Debug, thiserror::Error)]
#[error("university {id} does not exist")]
pub struct UniversityDoesNotExistError {
    pub(crate) id: SerialId,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUniversityException {
    #[error("univeristy name '{name}' is already in use")]
    UniversityAlreadyExists { name: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteUniversityException {
    #[error(transparent)]
    UniversityDoesNotExist(#[from] UniversityDoesNotExistError),
}
