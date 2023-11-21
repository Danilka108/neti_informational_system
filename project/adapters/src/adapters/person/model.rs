use anyhow::Context;
use app::person::Person;

use crate::adapters::IntoEntity;

pub struct PgPerson {
    pub id: i32,
}

impl IntoEntity<Person> for PgPerson {
    fn into_entity(self) -> Result<Person, anyhow::Error> {
        Ok(Person {
            id: self
                .id
                .try_into()
                .context("PgPerson 'id' must be non zero i32")?,
        })
    }
}

impl From<Person> for PgPerson {
    fn from(Person { id }: Person) -> Self {
        Self { id: id.into() }
    }
}
