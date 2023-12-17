use app::{university::University, SerialId};

use crate::adapters::IntoEntity;

#[derive(sqlx::FromRow)]
pub struct PgUniversity {
    pub id: SerialId,
    pub name: String,
}

impl IntoEntity<University> for PgUniversity {
    fn into_entity(self) -> Result<University, anyhow::Error> {
        Ok(University {
            id: self.id,
            name: self.name,
        })
    }
}
