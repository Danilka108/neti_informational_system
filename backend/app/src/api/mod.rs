mod password_encoder;
mod session_repository;
mod token_manager;
mod user_repository;

pub use password_encoder::PasswordEncoder;
pub use session_repository::SessionRepository;
pub use token_manager::TokenManager;
pub use user_repository::UserRepository;

#[derive(Debug, thiserror::Error)]
#[error("entity already exist")]
pub struct EntityAlreadyExistError;

#[derive(Debug, thiserror::Error)]
#[error("entity does not exist")]
pub struct EntityDoesNotExistError;

#[derive(Debug, thiserror::Error)]
#[error("entity not found")]
pub struct EntityNotFoundError;
