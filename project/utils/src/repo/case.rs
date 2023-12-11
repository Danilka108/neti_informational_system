use std::marker::PhantomData;

use crate::entity::EntityTrait;

use super::ex::{Exception, Kind};

pub struct Case<Entity, Fields> {
    kind: Kind,
    fields: Fields,
    _marker: PhantomData<Entity>,
}

pub struct Undefined;

impl Case<Undefined, Undefined> {
    pub fn does_not_exist<Entity>() -> Case<Entity, Undefined> {
        Case {
            kind: Kind::DoesNotExist,
            fields: Undefined,
            _marker: PhantomData,
        }
    }

    pub fn unique_constraint_violated<Entity>() -> Case<Entity, Undefined> {
        Case {
            kind: Kind::UniqueConstraintViolation,
            fields: Undefined,
            _marker: PhantomData,
        }
    }

    pub fn ref_constraint_violated<Entity>() -> Case<Entity, Undefined> {
        Case {
            kind: Kind::RefConstraintViolation,
            fields: Undefined,
            _marker: PhantomData,
        }
    }
}

impl<Entity: EntityTrait> Case<Entity, Undefined> {
    pub fn with_fields<F>(self, fields: F) -> Case<Entity, F>
    where
        F: IntoIterator<Item = Entity::Attr>,
    {
        Case {
            kind: self.kind,
            fields,
            _marker: PhantomData,
        }
    }
}

impl<Entity, Fields> Case<Entity, Fields>
where
    Entity: EntityTrait,
    Fields: IntoIterator<Item = Entity::Attr>,
{
    pub fn eq_to(self, repo_ex: &Exception<Entity>) -> bool {
        if self.kind != repo_ex.kind {
            return false;
        }

        for field in self.fields.into_iter() {
            if !repo_ex.fields.contains(&field) {
                return false;
            }
        }

        true
    }
}
