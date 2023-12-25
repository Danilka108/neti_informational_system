use utils::{entity::Id, outcome::Outcome};

use crate::{
    passport::{self, Gender, PassportNumber, PassportSeries},
    person, user,
};

pub struct PersonService {
    repo: person::BoxedRepo,
    passport_repo: passport::BoxedRepo,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PersonException {
    #[error("person already exist")]
    AlreadyExist,
    #[error("passport already exist")]
    PassportAlreadyExist,
}

pub struct PassportInfo {
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub date_of_birth: time::Date,
    pub date_of_issue: time::Date,
    pub number: PassportNumber,
    pub series: PassportSeries,
    pub gender: Gender,
}

impl PassportInfo {
    fn into_passport_entity(self, person_id: person::EntityId) -> passport::Entity {
        passport::Entity {
            id: Default::default(),
            person_id,
            first_name: self.first_name,
            last_name: self.last_name,
            patronymic: self.patronymic,
            date_of_birth: self.date_of_birth,
            date_of_issue: self.date_of_issue,
            number: self.number,
            series: self.series,
            gender: self.gender,
        }
    }
}

impl PersonService {
    pub async fn create(
        &mut self,
        user_id: user::EntityId,
        full_name: String,
    ) -> Outcome<person::Entity, PersonException> {
        if self.repo.find_by_user_id(user_id).await?.is_some() {
            return Outcome::Ex(PersonException::AlreadyExist);
        }

        let person = person::Entity {
            id: Default::default(),
            user_id,
            full_name,
        };

        let person = self.repo.save(person).await?;
        Outcome::Ok(person)
    }

    pub async fn add_passort(
        &mut self,
        person_id: person::EntityId,
        passport: PassportInfo,
    ) -> Outcome<passport::Entity, PersonException> {
        let is_already_exist = self
            .passport_repo
            .find_by_number_series(passport.number, passport.series)
            .await?
            .is_some();

        if is_already_exist {
            return Outcome::Ex(PersonException::PassportAlreadyExist);
        }

        let passport = self
            .passport_repo
            .save(passport.into_passport_entity(person_id))
            .await?;

        Outcome::Ok(passport)
    }
}
