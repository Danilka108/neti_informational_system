use app::class;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Classes {
    pub id: i32,
    pub curriculum_module_id: i32,
    pub kind_name: String,
}

impl From<Classes> for class::Entity {
    fn from(value: Classes) -> Self {
        class::Entity {
            id: Id::new(value.id),
            curriculum_module_id: Id::new(value.curriculum_module_id),
            kind_name: Id::new(value.kind_name),
        }
    }
}
