use app::university;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Universities {
    pub id: i32,
    pub name: String,
}

impl From<Universities> for university::Entity {
    fn from(value: Universities) -> Self {
        university::Entity {
            id: Id::new(value.id),
            name: value.name,
        }
    }
}
