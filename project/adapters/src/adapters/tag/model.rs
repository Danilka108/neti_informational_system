use app::{tag::Tag, SerialId};

use crate::adapters::IntoEntity;

pub struct PgTag {
    pub id: SerialId,
    pub name: String,
}

impl IntoEntity<Tag> for PgTag {
    fn into_entity(self) -> Result<Tag, anyhow::Error> {
        Ok(Tag {
            id: self.id,
            name: self.name,
        })
    }
}
