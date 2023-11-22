use app::{
    person::Person,
    subdivision::{Subdivision, SubdivisionMember, SubdivisionTag},
    tag::Tag,
    university::University,
    SerialId,
};

use crate::adapters::IntoEntity;

pub struct PgSubdivision {
    pub subdivision_id: SerialId,
    pub subdivision_name: String,
    pub university_id: SerialId,
    pub university_name: String,
}

impl IntoEntity<Subdivision> for PgSubdivision {
    fn into_entity(self) -> Result<Subdivision, anyhow::Error> {
        Ok(Subdivision {
            id: self.subdivision_id,
            name: self.subdivision_name,
            university: University {
                id: self.university_id,
                name: self.university_name,
            },
        })
    }
}

pub struct PgSubdivisionTag {
    pub subdivision_id: SerialId,
    pub subdivision_name: String,
    pub university_id: SerialId,
    pub university_name: String,
    pub tag_id: SerialId,
    pub tag_name: String,
}

impl IntoEntity<SubdivisionTag> for PgSubdivisionTag {
    fn into_entity(self) -> Result<SubdivisionTag, anyhow::Error> {
        Ok(SubdivisionTag(
            Subdivision {
                id: self.subdivision_id,
                name: self.subdivision_name,
                university: University {
                    id: self.university_id,
                    name: self.university_name,
                },
            },
            Tag {
                id: self.tag_id,
                name: self.tag_name,
            },
        ))
    }
}

pub struct PgSubdivisionMember {
    pub subdivision_id: SerialId,
    pub subdivision_name: String,
    pub university_id: SerialId,
    pub university_name: String,
    pub person_id: SerialId,
    pub role: String,
}

impl IntoEntity<SubdivisionMember> for PgSubdivisionMember {
    fn into_entity(self) -> Result<SubdivisionMember, anyhow::Error> {
        Ok(SubdivisionMember {
            id: (
                Subdivision {
                    id: self.subdivision_id,
                    name: self.subdivision_name,
                    university: University {
                        id: self.university_id,
                        name: self.university_name,
                    },
                },
                Person { id: self.person_id },
            ),
            role: self.role,
        })
    }
}
