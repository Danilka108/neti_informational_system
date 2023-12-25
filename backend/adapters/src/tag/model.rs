use app::tag;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Tags {
    pub name: String,
}

impl From<Tags> for tag::Entity {
    fn from(value: Tags) -> Self {
        tag::Entity {
            name: Id::new(value.name),
        }
    }
}
