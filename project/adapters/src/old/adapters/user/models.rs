use app::user::{HashedPassword, Role, User};
use app::{Ref, SerialId};

use crate::adapters::IntoEntity;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub enum PgRole {
    Admin,
}

pub struct PgUser {
    pub id: SerialId,
    pub email: String,
    pub hashed_password: String,
    pub role: PgRole,
}

impl From<Role> for PgRole {
    fn from(value: Role) -> PgRole {
        match value {
            Role::Admin => Self::Admin,
        }
    }
}

impl From<PgRole> for Role {
    fn from(value: PgRole) -> Self {
        match value {
            PgRole::Admin => Self::Admin,
        }
    }
}

impl IntoEntity<User> for PgUser {
    fn into_entity(self) -> Result<User, anyhow::Error> {
        Ok(User {
            id: Ref::from(self.id),
            email: self.email,
            hashed_password: HashedPassword(self.hashed_password),
            role: self.role.into(),
        })
    }
}
