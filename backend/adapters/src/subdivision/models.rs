use app::subdivision;
use sqlx::FromRow;
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Subdivisions {
    pub id: i32,
    pub name: String,
    pub university_id: i32,
}

#[derive(FromRow)]
pub struct JoinRow {
    #[sqlx(flatten)]
    pub subdivision: Subdivisions,
    #[sqlx(flatten)]
    pub tag: SubdivisionTags,
    #[sqlx(flatten)]
    pub member: SubdivisionMembers,
}

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct SubdivisionTags {
    pub subdivision_id: i32,
    pub tag_name: String,
}

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct SubdivisionMembers {
    pub subdivision_id: i32,
    pub person_id: i32,
    pub role: String,
}

impl Subdivisions {
    pub fn into_entity(
        self,
        tags: Vec<SubdivisionTags>,
        members: Vec<SubdivisionMembers>,
    ) -> subdivision::Entity {
        subdivision::Entity {
            id: Id::new(self.id),
            name: self.name,
            university_id: Id::new(self.university_id),
            tags: tags.into_iter().map(|v| Id::new(v.tag_name)).collect(),
            members: members.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SubdivisionMembers> for subdivision::Member {
    fn from(value: SubdivisionMembers) -> Self {
        subdivision::Member {
            person_id: Id::new(value.person_id),
            role: value.role,
        }
    }
}
