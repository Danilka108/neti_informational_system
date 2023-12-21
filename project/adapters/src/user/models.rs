use app::user;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub password: String,
}

impl From<Users> for user::Entity {
    fn from(value: Users) -> Self {
        user::Entity {
            id: Id::new(value.id),
            email: value.email.into(),
            password: user::HashedPassword {
                value: value.password.into(),
            },
        }
    }
}
