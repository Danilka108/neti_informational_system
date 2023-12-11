use utils::repo::{case::Case, ex::FromRepoEx};

use super::{Entity, EntityAttr};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("tag  does not exist")]
    DoesNotExist,
    #[error("tag already exists")]
    AlreadyExists,
}

impl FromRepoEx<Entity> for Exception {
    fn from_repo_ex<Ok>(repo_ex: &utils::repo::ex::Exception<Entity>) -> Option<Self> {
        if Case::does_not_exist()
            .with_fields([EntityAttr::Name])
            .eq_to(&repo_ex)
        {
            return Some(Exception::DoesNotExist);
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Name])
            .eq_to(&repo_ex)
        {
            return Some(Exception::AlreadyExists);
        }

        None
    }
}
