use di::{Module, Provide};

use crate::{
    base_repo::{BaseRepo, BaseRepoException},
    university, AdaptersModule, AppModule, EntityTrait, FieldTrait, Outcome,
};

mod create;
mod delete;
mod get;
mod get_by_university;

pub type Id = crate::Id<Entity>;

#[derive(Debug)]
pub struct Entity {
    pub id: Id,
    pub name: String,
    pub university_id: university::Id,
}

pub enum Field {
    Id,
    Name,
    UniversityId,
}

impl EntityTrait for Entity {
    const NAME: &'static str = "university";

    type Field = Field;
    type IdValue = i32;

    fn get_field_value(&self, field: Self::Field) -> String {
        match field {
            Field::Id => self.id.value.to_string(),
            Field::Name => self.name.clone(),
            Field::UniversityId => self.university_id.value.to_string(),
        }
    }
}

impl FieldTrait for Field {
    fn name(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::UniversityId => "university_id",
        }
    }
}

#[async_trait::async_trait]
pub trait Repo: BaseRepo<Entity> {
    async fn list_by_university(
        &self,
        university_id: university::Id,
    ) -> Outcome<Vec<Entity>, BaseRepoException<Entity>>;
}

pub type BoxedRepo = Box<dyn Repo>;

pub struct Service {
    repo: BoxedRepo,
    university_service: university::Service,
}

impl<A: AdaptersModule> Provide<Service> for AppModule<A> {
    fn provide(&self) -> Service {
        Service {
            repo: self.adapters.resolve(),
            university_service: self.resolve(),
        }
    }
}
