use app::person::Person;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PgPerson {
    pub id: i32,
}

impl From<Person> for PgPerson {
    fn from(Person { id }: Person) -> Self {
        Self { id }
    }
}

impl From<PgPerson> for Person {
    fn from(PgPerson { id }: PgPerson) -> Self {
        Self { id }
    }
}
