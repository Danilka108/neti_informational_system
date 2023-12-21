use std::{fmt::Display, str::FromStr};

use app::passport::{self, Gender};
use sqlx::{FromRow, Type};
use utils::entity::Id;

#[derive(Clone, Debug, FromRow)]
#[sea_query::enum_def]
pub struct Passports {
    pub id: i32,
    pub person_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub date_of_birth: time::Date,
    pub date_of_issue: time::Date,
    pub number: String,
    pub series: String,
    pub gender: PgGender,
}

impl From<Passports> for passport::Entity {
    fn from(value: Passports) -> Self {
        passport::Entity {
            id: Id::new(value.id),
            person_id: Id::new(value.person_id),
            first_name: value.first_name,
            last_name: value.last_name,
            patronymic: value.patronymic,
            date_of_birth: value.date_of_birth,
            date_of_issue: value.date_of_issue,
            number: FromStr::from_str(&value.number).unwrap(),
            series: FromStr::from_str(&value.series).unwrap(),
            gender: value.gender.into(),
        }
    }
}

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "lowercase")]
pub enum PgGender {
    Male,
    Female,
}

impl Display for PgGender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Male => "male",
                Self::Female => "female",
            }
        )
    }
}

impl From<Gender> for PgGender {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => PgGender::Male,
            Gender::Female => PgGender::Female,
        }
    }
}

impl From<PgGender> for Gender {
    fn from(value: PgGender) -> Self {
        match value {
            PgGender::Male => Gender::Male,
            PgGender::Female => Gender::Female,
        }
    }
}
