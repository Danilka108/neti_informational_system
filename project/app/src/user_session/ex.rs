use utils::repo::{case::Case, ex::FromRepoEx};

use super::{Entity, EntityAttr};

#[derive(Debug, thiserror::Error)]
pub enum Exception {
    #[error("session does not exist")]
    DoesNotExist,
    #[error("session already exist")]
    AlreadyExist,
    #[error("sessions limit reached")]
    LimitReached,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("session expired")]
    Expired,
    #[error("user does not exist")]
    UserDoesNotExist,
}

impl FromRepoEx<Entity> for Exception {
    fn from_repo_ex<Ok>(repo_ex: &utils::repo::ex::Exception<Entity>) -> Option<Self> {
        if Case::does_not_exist()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Exception::DoesNotExist.into();
        }

        if Case::unique_constraint_violated()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Exception::AlreadyExist.into();
        }

        if Case::ref_constraint_violated()
            .with_fields([EntityAttr::Id])
            .eq_to(&repo_ex)
        {
            return Exception::UserDoesNotExist.into();
        }

        None
    }
}
