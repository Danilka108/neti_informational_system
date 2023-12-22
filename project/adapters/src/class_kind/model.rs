use app::class_kind;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct ClassKinds {
    pub name: String,
}

impl From<ClassKinds> for class_kind::Entity {
    fn from(value: ClassKinds) -> Self {
        class_kind::Entity {
            name: Id::new(value.name),
        }
    }
}
