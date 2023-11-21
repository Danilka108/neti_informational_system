use std::num::NonZeroI32;

use crate::{person::GetPersonException, university::exceptions::UniversityDoesNotExistError};

#[derive(Debug, thiserror::Error)]
#[error("the subdivision name '{subdivision_name}' for the university '{university_name}' is already in use")]
pub struct NameIsAlreadyInUseError {
    pub(crate) university_name: String,
    pub(crate) subdivision_name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("subdivision {id} does not exist")]
pub struct SubdivisionDoesNotExistError {
    pub(crate) id: NonZeroI32,
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
    #[error("tag '{tag_name}' is already exist for subdivision {subdivision_id}")]
    TagIsAlreadyExist {
        subdivision_id: NonZeroI32,
        tag_name: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSubdivisionTagException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error("tag {tag_name} does not exist for subdivision {subdivision_id}")]
    TagDoesNotExist {
        tag_name: String,
        subdivision_id: NonZeroI32,
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
    #[error("member {person_id} is already exist for subdivision {subdivision_id}")]
    MemberIsAlreadyExist {
        person_id: NonZeroI32,
        subdivision_id: NonZeroI32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateSubdivisionMemberRoleException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error(transparent)]
    PersonDoesNotExist(#[from] GetPersonException),
    #[error("person {person_id} is not a member of subdivision {subdivision_id}")]
    MemberDoesNotExist {
        person_id: NonZeroI32,
        subdivision_id: NonZeroI32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubdivisionMemberException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error(transparent)]
    PersonDoesNotExist(#[from] GetPersonException),
    #[error("person {person_id} is not a member of subdivision {subdivision_id}")]
    MemberDoesNotExist {
        person_id: NonZeroI32,
        subdivision_id: NonZeroI32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSubdivisionMemberException {
    #[error(transparent)]
    SubdivisionDoesNotExist(#[from] SubdivisionDoesNotExistError),
    #[error("member {person_id} does not exist for subdivision {subdivision_id}")]
    MemberDoesNotExist {
        person_id: NonZeroI32,
        subdivision_id: NonZeroI32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum GetSubdivisionsByUniversityException {
    #[error(transparent)]
    UniversityDoesNotExist(#[from] UniversityDoesNotExistError),
}
