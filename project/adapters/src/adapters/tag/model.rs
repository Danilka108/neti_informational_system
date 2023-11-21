use anyhow::Context;
use app::tag::Tag;

use crate::adapters::IntoEntity;

pub struct PgTag {
    pub id: i32,
    pub name: String,
}

impl IntoEntity<Tag> for PgTag {
    fn into_entity(self) -> Result<Tag, anyhow::Error> {
        Ok(Tag {
            id: self
                .id
                .try_into()
                .context("'id' of PgTag must be non zero i32")?,
            name: self.name,
        })
    }
}
