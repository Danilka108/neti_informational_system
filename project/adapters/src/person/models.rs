use app::{person, user};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Persons {
    pub id: i32,
    pub user_id: i32,
}

impl From<Persons> for person::Entity {
    fn from(value: Persons) -> Self {
        person::Entity {
            id: person::EntityId::new(value.id),
            user_id: user::EntityId::new(value.user_id),
        }
    }
}
