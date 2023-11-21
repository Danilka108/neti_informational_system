pub use crate::person::{DynPersonRepository, PersonRepository};
pub use crate::session::{DynSessionRepository, SessionRepository, SessionTTL, SessionsMaxNumber};
pub use crate::subdivision::{
    BoxedSubdivisionMemberRepository, BoxedSubdivisionRepository, BoxedSubdivisionTagRepository,
    SubdivisionMemberRepository, SubdivisionRepository, SubdivisionTagRepository,
};
pub use crate::tag::{BoxedTagRepository, TagRepository};
pub use crate::token::{
    AccessTokenEngine, AccessTokenTTL, DynAccessTokenEngine, DynRefreshTokenGenerator,
    RefreshTokenGenerator,
};
pub use crate::university::{BoxedUniversityRepository, UniversityRepository};
pub use crate::user::{DynPasswordHasher, DynUserRepository, PasswordHasher, UserRepository};

#[derive(Default, Debug, thiserror::Error)]
#[error("entity already exist")]
pub struct EntityAlreadyExistError;

#[derive(Default, Debug, thiserror::Error)]
#[error("entity does not exist")]
pub struct EntityDoesNotExistError;

#[derive(Default, Debug, thiserror::Error)]
#[error("entity not found")]
pub struct EntityNotFoundError;

#[derive(Default, Debug, thiserror::Error)]
#[error("uniqual value error")]
pub struct UniqualValueError;
