use std::fmt::Display;

use crate::{
    entity::{AttrTrait, EntityTrait},
    outcome::Outcome,
};

pub struct Exception<E: EntityTrait> {
    pub(super) kind: Kind,
    pub(super) fields: Vec<E::Attr>,
}

pub trait FromRepoEx<Entity: EntityTrait>: Sized {
    fn from_repo_ex<Ok>(repo_ex: &Exception<Entity>) -> Option<Self>;
}

impl<Entity: EntityTrait, Ok> Outcome<Ok, Exception<Entity>> {
    pub fn map_repo_ex<Ex>(self) -> Outcome<Ok, Ex>
    where
        Ex: FromRepoEx<Entity>,
        Entity::Attr: Send + Sync,
        Entity: 'static,
    {
        match self {
            Outcome::Ok(ok) => Outcome::Ok(ok),
            Outcome::Ex(repo_ex) => match Ex::from_repo_ex::<Ok>(&repo_ex) {
                Some(ex) => Outcome::Ex(ex),
                None => Outcome::Error(anyhow::Error::new(repo_ex)),
            },
            Outcome::Error(err) => Outcome::Error(err),
        }
    }
}

struct DisplayableField<Entity: EntityTrait>(Entity::Attr);

impl<Entity: EntityTrait> Display for DisplayableField<Entity> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.0.name())
        write!(f, "")
    }
}

impl<E: EntityTrait> Exception<E> {
    fn fields_to_debug(&self) -> Vec<String> {
        // self.fields.iter().map(|f| f.name().to_owned()).collect()
        self.fields.iter().map(|f| "".to_owned()).collect()
    }
}

impl<E: EntityTrait> std::fmt::Debug for Exception<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Exception")
            .field("kind", &self.kind)
            .field("fields", &self.fields_to_debug())
            .finish()
    }
}

impl<E> Display for Exception<E>
where
    E: EntityTrait,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::DoesNotExist => write!(
                f,
                "entity {} does not exist (couldn't find the entity searched by fields {:?})",
                // E::NAME,
                "",
                self.fields_to_debug(),
            ),
            Kind::AlreadyExist => write!(
                f,
                "entity {} already exist (couldn't update the entity searched by fields {:?})",
                // E::NAME,
                "",
                self.fields_to_debug(),
            ),
            Kind::UniqueConstraintViolation => write!(
                f,
                "unique constraint violation of {} entity (unique constraint of the set of fields {:?} violated)",
                // E::NAME,
                "",
                self.fields_to_debug(),
            ),
            Kind::RefConstraintViolation => write!(
                f,
                "reference constraint violation of {} entity (reference constraint of the set of fields {:?} violated)",
                // E::NAME,
                "",
                self.fields_to_debug(),
            ),
            Kind::CheckConstraintViolation => write!(
                f,
                "check constraint violation of {} entity (check constraint of the set of fields {:?} violated)",
                // E::NAME,
                "",
                self.fields_to_debug(),
            ),
        }
    }
}

impl<Entity> std::error::Error for Exception<Entity> where Entity: EntityTrait {}

#[derive(Debug, PartialEq)]
pub(crate) enum Kind {
    DoesNotExist,
    AlreadyExist,
    UniqueConstraintViolation,
    RefConstraintViolation,
    CheckConstraintViolation,
}

impl<E: EntityTrait> Exception<E> {
    /// Use if couldn't find the entity searched by fields 'with_fields'
    pub fn does_not_exist<F>(with_fields: F) -> Self
    where
        F: IntoIterator<Item = E::Attr>,
    {
        Self {
            kind: Kind::DoesNotExist,
            fields: with_fields.into_iter().collect(),
        }
    }

    /// Use if couldn't update the entity searched by fields 'with_fields'
    pub fn already_exist<F>(with_fields: F) -> Self
    where
        F: IntoIterator<Item = E::Attr>,
    {
        Self {
            kind: Kind::AlreadyExist,
            fields: with_fields.into_iter().collect(),
        }
    }

    /// Use if unique constraint of the set of fields 'unique_fields' violated
    pub fn unique_constraint_violation<'i, F>(unique_fields: F) -> Self
    where
        F: IntoIterator<Item = E::Attr>,
    {
        Self {
            kind: Kind::UniqueConstraintViolation,
            fields: unique_fields.into_iter().collect(),
        }
    }

    /// Use if reference constraint of the set of fields 'ref_fields' violated
    pub fn ref_constraint_violation<F>(ref_fields: F) -> Self
    where
        F: IntoIterator<Item = E::Attr>,
    {
        Self {
            kind: Kind::RefConstraintViolation,
            fields: ref_fields.into_iter().collect(),
        }
    }

    /// Use if check constraint of the set of fields 'check_fields' violated
    pub fn check_constraint_violation<F>(check_fields: F) -> Self
    where
        F: IntoIterator<Item = E::Attr>,
    {
        Self {
            kind: Kind::CheckConstraintViolation,
            fields: check_fields.into_iter().collect(),
        }
    }
}
