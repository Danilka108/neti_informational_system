use utils::repo::{case::Case, ex::FromRepoEx};

use super::{Entity, EntityAttr};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("user does not exist")]
    DoesNotExist,
    #[error("user is already is use")]
    EmailAlreadyInUse,
    #[error("user already exist")]
    AlreadyExist,
    #[error("user person password")]
    InvalidPassword,
}

impl FromRepoEx<Entity> for Exception {
    fn from_repo_ex<Ok>(repo_ex: &utils::repo::ex::Exception<Entity>) -> Option<Self> {
        if Case::does_not_exist()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Some(Exception::DoesNotExist);
        }

        if Case::does_not_exist()
            .with_fields([EntityAttr::Email])
            .eq_to(&repo_ex)
        {
            return Some(Exception::DoesNotExist);
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Email])
            .eq_to(&repo_ex)
        {
            return Some(Exception::EmailAlreadyInUse);
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Some(Exception::AlreadyExist);
        }

        None
    }
}
