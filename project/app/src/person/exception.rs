use utils::repo::case::Case;
use utils::repo::ex::FromRepoEx;

use super::{Entity, EntityAttr};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("person does not exist")]
    DoesNotExist,
    #[error("person already exist")]
    AlreadyExists,
    #[error("the user is already represented by another person")]
    UserAlreadyRepresented,
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
            .with_fields([EntityAttr::UserId])
            .eq_to(&repo_ex)
        {
            return Some(Exception::UserAlreadyRepresented);
        }

        None
    }
}
