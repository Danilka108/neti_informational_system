use app::discipline;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Disciplines {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
}

impl From<Disciplines> for discipline::Entity {
    fn from(value: Disciplines) -> Self {
        discipline::Entity {
            id: Id::new(value.id),
            name: value.name,
            department_id: Id::new(value.department_id),
        }
    }
}
