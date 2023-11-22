mod password_hasher;
mod repository;
mod service;

pub use password_hasher::PasswordHasher;
pub use repository::UserRepository;
pub use service::{AuthenticateUserException, CreateUserException, UserService};

use crate::{person::Person, Ref, SerialId};

pub type DynUserRepository = Box<dyn UserRepository + Send + Sync>;
pub type DynPasswordHasher = Box<dyn PasswordHasher + Send + Sync>;

#[derive(Clone, Debug, Hash)]
pub struct User {
    pub id: Ref<SerialId, Person>,
    pub email: String,
    pub role: Role,
    pub hashed_password: HashedPassword,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for User {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Role {
    Admin,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HashedPassword(pub String);
