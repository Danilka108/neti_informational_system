mod password_hasher;
mod repository;
mod service;

use serde::{Deserialize, Serialize};

pub use password_hasher::PasswordHasher;
pub use repository::UserRepository;
pub use service::{AuthenticateUserError, CreateUserError, UserService};

pub type DynUserRepository = Box<dyn UserRepository + Send + Sync>;
pub type DynPasswordHasher = Box<dyn PasswordHasher + Send + Sync>;

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub struct User<Id = i32> {
    pub id: Id,
    pub person_id: i32,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Role {
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct HashedPassword(pub String);
