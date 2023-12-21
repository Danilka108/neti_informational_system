use utils::repo::case::Case;
use utils::repo::ex::FromRepoEx;

use super::{Entity, EntityAttr};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("subdivision does not exist")]
    DoesNotExist,
    #[error("subdivision already exists")]
    AlreadyExists,
    #[error("subdivision already exists at the university")]
    AlreadyExistsAtTheUniversity,
    #[error("university does not exists")]
    UniversityDoesNotExists,
    #[error("the subdivision does not have the tag")]
    TagDoesNotFound,
    #[error("the tag already exists for the subdivision")]
    TagAlreadyExist,
    #[error("tag or subdivision does not exist")]
    TagOrSubdivisionDoesNotExist,
    #[error("the subdivision does not have the member")]
    MemberDoesNotFound,
    #[error("person or subdivision does not exist")]
    PersonOrSubdivisionDoesNotExist,
    #[error("the tag already exists for the member")]
    MemberAlreadyExist,
}

impl FromRepoEx<Entity> for Exception {
    fn from_repo_ex<Ok>(repo_ex: &utils::repo::ex::Exception<Entity>) -> Option<Self> {
        if Case::does_not_exist()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Some(Exception::DoesNotExist);
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Some(Exception::AlreadyExists);
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::UniversityId])
            .eq_to(&repo_ex)
        {
            return Some(Exception::UniversityDoesNotExists);
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Name, EntityAttr::UniversityId])
            .eq_to(&repo_ex)
        {
            return Some(Exception::AlreadyExistsAtTheUniversity);
        }

        if Case::does_not_exist()
            .with_fields([EntityAttr::Tags])
            .eq_to(&repo_ex)
        {
            return Exception::TagDoesNotFound.into();
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Tags])
            .eq_to(&repo_ex)
        {
            return Exception::TagAlreadyExist.into();
        }

        if Case::ref_constraint_violated()
            .with_fields([EntityAttr::Tags])
            .eq_to(&repo_ex)
        {
            return Exception::TagOrSubdivisionDoesNotExist.into();
        }

        if Case::does_not_exist()
            .with_fields([EntityAttr::Members])
            .eq_to(&repo_ex)
        {
            return Exception::MemberDoesNotFound.into();
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Members])
            .eq_to(&repo_ex)
        {
            return Exception::MemberAlreadyExist.into();
        }

        if Case::ref_constraint_violated()
            .with_fields([EntityAttr::Members])
            .eq_to(&repo_ex)
        {
            return Exception::PersonOrSubdivisionDoesNotExist.into();
        }

        None
    }
}
