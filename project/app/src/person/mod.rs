use di::Provide;

use crate::{base_repo::BaseRepo, AdaptersModule, AppModule, EntityTrait, FieldTrait};

mod create;
mod delete;
mod get;

pub type Id = crate::Id<Entity>;

#[derive(Debug)]
pub struct Entity {
    pub id: Id,
    pub name: String,
}

pub enum Field {
    Id,
    Name,
}

#[async_trait::async_trait]
pub trait Repo: BaseRepo<Entity> {}

pub type BoxedRepo = Box<dyn Repo>;

pub struct Service {
    repo: BoxedRepo,
}

impl<A: AdaptersModule> Provide<Service> for AppModule<A> {
    fn provide(&self) -> Service {
        Service {
            repo: self.adapters.resolve(),
        }
    }
}

impl EntityTrait for Entity {
    const NAME: &'static str = "university";

    type Field = Field;
    type IdValue = i32;

    fn get_field_value(&self, field: Self::Field) -> String {
        match field {
            Field::Id => self.id.value.to_string(),
            Field::Name => self.name.clone(),
        }
    }
}

impl FieldTrait for Field {
    fn name(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
        }
    }
}
