use std::num::NonZeroI32;

use crate::university::exceptions::UniversityDoesNotExistError;

#[derive(Debug, thiserror::Error)]
#[error("the subdivision name '{subdivision_name}' for the university '{university_name}' is already in use")]
pub struct NameIsAlreadyInUseError {
    pub(crate) university_name: String,
    pub(crate) subdivision_name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("subdivision '{name}' does not exist")]
pub struct SubdivisionDoesNotExistError {
    pub(crate) name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSubdivisionException {
    #[error(transparent)]
    NameIsAlreadyInUse(#[from] NameIsAlreadyInUseError),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSubdivisionException {
    #[error(transparent)]
    DoesNotExist(#[from] SubdivisionDoesNotExistError),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateSubdivisionNameException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error(transparent)]
    NameIsAlreadyInUse(#[from] NameIsAlreadyInUseError),
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubdivisionTagsException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
}

#[derive(Debug, thiserror::Error)]
pub enum AddSubdivisionTagException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error("tag '{tag_name}' is already exist for subdivision '{subdivision_name}'")]
    TagIsAlreadyExist {
        subdivision_name: String,
        tag_name: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSubdivisionTagException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error("tag {tag_name} does not exist for subdivision '{subdivision_name}'")]
    TagDoesNotExist {
        tag_name: String,
        subdivision_name: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubdivisionMembersException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
}

#[derive(Debug, thiserror::Error)]
pub enum AddSubdivisionMemberException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error("member {person_id} is already exist for subdivision '{subdivision_name}'")]
    MemberIsAlreadyExist {
        person_id: NonZeroI32,
        subdivision_name: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSubdivisionMemberException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error("member {person_id} does not exist for subdivision '{subdivision_name}'")]
    MemberDoesNotExist {
        person_id: NonZeroI32,
        subdivision_name: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubdivisionsByUniversityException {
    #[error(transparent)]
    UniversityDoesNotExist(#[from] UniversityDoesNotExistError),
}
