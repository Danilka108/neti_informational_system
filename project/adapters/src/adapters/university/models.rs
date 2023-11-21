use anyhow::Context;
use app::university::University;

use crate::adapters::IntoEntity;

#[derive(sqlx::FromRow)]
pub struct PgUniversity {
    pub id: i32,
    pub name: String,
}

impl IntoEntity<University> for PgUniversity {
    fn into_entity(self) -> Result<University, anyhow::Error> {
        Ok(University {
            id: self
                .id
                .try_into()
                .context("'id' of PgUniversity must be non zero i32")?,
            name: self.name,
        })
    }
}
