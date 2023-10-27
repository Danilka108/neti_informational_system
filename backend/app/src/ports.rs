pub use crate::person::{DynPersonRepository, PersonRepository};
pub use crate::session::{DynSessionRepository, SessionRepository, SessionTTL, SessionsMaxNumber};
pub use crate::token::{
    AccessTokenEngine, AccessTokenTTL, DynAccessTokenEngine, DynRefreshTokenGenerator,
    RefreshTokenGenerator,
};
pub use crate::user::{DynPasswordHasher, DynUserRepository, PasswordHasher, UserRepository};

#[derive(Debug, thiserror::Error)]
#[error("entity already exist")]
pub struct EntityAlreadyExistError;

#[derive(Debug, thiserror::Error)]
#[error("entity does not exist")]
pub struct EntityDoesNotExistError;
