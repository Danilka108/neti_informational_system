use app::curriculum_module;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct CurriculumModules {
    pub id: i32,
    pub discipline_id: i32,
    pub curriculum_id: i32,
    pub semester: i32,
}

impl From<CurriculumModules> for curriculum_module::Entity {
    fn from(value: CurriculumModules) -> Self {
        curriculum_module::Entity {
            id: Id::new(value.id),
            discipline_id: Id::new(value.discipline_id),
            curriculum_id: Id::new(value.curriculum_id),
            semester: value.semester,
        }
    }
}
