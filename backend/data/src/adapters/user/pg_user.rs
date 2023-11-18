use app::user::{HashedPassword, Role, User};
use app::Ref;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub enum PgRole {
    Admin,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PgUser {
    pub id: i32,
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

impl From<PgUser> for User {
    fn from(
        PgUser {
            id,
            email,
            hashed_password,
            role,
        }: PgUser,
    ) -> Self {
        Self {
            id: Ref::from(id),
            email,
            hashed_password: HashedPassword(hashed_password),
            role: role.into(),
        }
    }
}
