use crate::person;

mod number;
mod repo;

pub use repo::Repo;
pub type BoxedRepo = Box<dyn Repo + Send + Sync>;

pub use number::{
    InvalidPassportNumberError, InvalidPassportSeriesError, PassportNumber, PassportSeries,
};

#[utils::entity::entity]
pub struct Entity {
    #[id]
    pub id: i32,
    pub person_id: person::EntityId,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub date_of_birth: time::Date,
    pub date_of_issue: time::Date,
    pub number: PassportNumber,
    pub series: PassportSeries,
    pub gender: Gender,
}

pub enum Gender {
    Male,
    Female,
}
