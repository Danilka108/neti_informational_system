use app::{person::Person, SerialId};

use crate::adapters::IntoEntity;

pub struct PgPerson {
    pub id: SerialId,
}

impl IntoEntity<Person> for PgPerson {
    fn into_entity(self) -> Result<Person, anyhow::Error> {
        Ok(Person { id: self.id })
    }
}

impl From<Person> for PgPerson {
    fn from(Person { id }: Person) -> Self {
        Self { id }
    }
}
