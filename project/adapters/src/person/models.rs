use app::user;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Persons {
    pub id: i32,
    pub email: String,
    pub password: String,
}

impl From<Persons> for user::Entity {
    fn from(value: Persons) -> Self {
        user::Entity {
            id: Id::new(value.id),
            email: value.email,
            password: user::HashedPassword {
                value: value.password,
            },
        }
    }
}
