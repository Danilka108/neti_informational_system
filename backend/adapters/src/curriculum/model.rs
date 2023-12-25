use app::curriculum;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Debug, Clone, FromRow)]
#[sea_query::enum_def]
pub struct Curriculums {
    pub id: i32,
    pub name: String,
}

impl From<Curriculums> for curriculum::Entity {
    fn from(value: Curriculums) -> Self {
        curriculum::Entity {
            id: Id::new(value.id),
            name: value.name,
        }
    }
}
