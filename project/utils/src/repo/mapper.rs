use std::marker::PhantomData;

use crate::{entity::EntityTrait, outcome::Outcome};

use super::ex::{Exception, Kind};

impl<Ok, Entity: EntityTrait> Outcome<Ok, Exception<Entity>> {
    pub fn into_mapper(self) -> InitialMapper<Entity, Ok> {
        InitialMapper { outcome: self }
    }
}

pub enum MapResult<Entity: EntityTrait, Ex> {
    Mapped(Ex),
    Skipped(Exception<Entity>),
}

pub trait Mapper<Entity: EntityTrait, Ok, Ex>: Sized {
    type Res;

    fn map(self) -> Outcome<Ok, Self::Res>;

    fn case<Fields>(
        self,
        case: MapCase<Entity, Fields, Ex>,
    ) -> CaseMapper<Entity, Self, Fields, Ex> {
        CaseMapper { inner: self, case }
    }

    fn default<F>(self, f: F) -> DefaultMapper<Self, F> {
        DefaultMapper { inner: self, f }
    }

    fn default_context<C>(
        self,
        ctx: C,
    ) -> DefaultMapper<Self, Box<dyn FnOnce(Exception<Entity>) -> Result<Ex, anyhow::Error>>>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        Entity::Attr: Send + Sync,
        Entity: 'static,
    {
        DefaultMapper {
            inner: self,
            f: Box::new(|repo_ex| Err(anyhow::Error::new(repo_ex).context(ctx))),
        }
    }
}

pub struct InitialMapper<Entity: EntityTrait, Ok> {
    outcome: Outcome<Ok, Exception<Entity>>,
}

impl<Entity, Ok, Ex> Mapper<Entity, Ok, Ex> for InitialMapper<Entity, Ok>
where
    Entity: EntityTrait,
{
    type Res = MapResult<Entity, Ex>;

    fn map(self) -> Outcome<Ok, MapResult<Entity, Ex>> {
        self.outcome.map_ex(MapResult::Skipped)
    }
}

pub struct CaseMapper<Entity, Inner, Fields, Ex> {
    case: MapCase<Entity, Fields, Ex>,
    inner: Inner,
}

impl<Entity, Inner, Fields, Ok, Ex> Mapper<Entity, Ok, Ex> for CaseMapper<Entity, Inner, Fields, Ex>
where
    Entity: EntityTrait,
    Inner: Mapper<Entity, Ok, Ex, Res = MapResult<Entity, Ex>>,
    Fields: IntoIterator<Item = Entity::Attr>,
{
    type Res = MapResult<Entity, Ex>;

    fn map(self) -> Outcome<Ok, MapResult<Entity, Ex>> {
        let repo_ex = match self.inner.map() {
            Outcome::Ok(ok) => return Outcome::Ok(ok),
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Ex(map_res) => match map_res {
                MapResult::Mapped(ex) => return Outcome::Ex(MapResult::Mapped(ex)),
                MapResult::Skipped(repo_ex) => repo_ex,
            },
        };

        if self.case.kind != repo_ex.kind {
            return Outcome::Ex(MapResult::Skipped(repo_ex));
        }

        for field in self.case.fields {
            if !repo_ex.fields.contains(&field) {
                return Outcome::Ex(MapResult::Skipped(repo_ex));
            }
        }

        Outcome::Ex(MapResult::Mapped(self.case.ex))
    }
}

pub struct DefaultMapper<Inner, F> {
    f: F,
    inner: Inner,
}

impl<Entity, Inner, Ok, Ex, F> Mapper<Entity, Ok, Ex> for DefaultMapper<Inner, F>
where
    Entity: EntityTrait,
    Inner: Mapper<Entity, Ok, Ex, Res = MapResult<Entity, Ex>>,
    F: FnOnce(Exception<Entity>) -> Result<Ex, anyhow::Error>,
{
    type Res = Ex;

    fn map(self) -> Outcome<Ok, Self::Res> {
        match self.inner.map() {
            Outcome::Ok(ok) => Outcome::Ok(ok),
            Outcome::Error(err) => Outcome::Error(err),
            Outcome::Ex(map_res) => match map_res {
                MapResult::Mapped(ex) => Outcome::Ex(ex),
                MapResult::Skipped(repo_ex) => match (self.f)(repo_ex) {
                    Ok(ex) => Outcome::Ex(ex),
                    Err(err) => Outcome::Error(err),
                },
            },
        }
    }
}

pub struct MapCase<Entity, Fields, Ex> {
    kind: Kind,
    fields: Fields,
    ex: Ex,
    _marker: PhantomData<Entity>,
}

pub struct Undefined;

impl MapCase<Undefined, Undefined, Undefined> {
    pub fn if_does_not_exist<Entity>() -> MapCase<Entity, Undefined, Undefined> {
        MapCase {
            kind: Kind::DoesNotExist,
            fields: Undefined,
            ex: Undefined,
            _marker: PhantomData,
        }
    }

    // pub fn if_already_exist<Entity>() -> MapCase<Entity, Undefined, Undefined> {
    //     MapCase {
    //         kind: Kind::AlreadyExist,
    //         fields: Undefined,
    //         ex: Undefined,
    //         _marker: PhantomData,
    //     }
    // }

    pub fn if_unique_constraint_violated<Entity>() -> MapCase<Entity, Undefined, Undefined> {
        MapCase {
            kind: Kind::UniqueConstraintViolation,
            fields: Undefined,
            ex: Undefined,
            _marker: PhantomData,
        }
    }

    pub fn if_ref_constraint_violated<Entity>() -> MapCase<Entity, Undefined, Undefined> {
        MapCase {
            kind: Kind::RefConstraintViolation,
            fields: Undefined,
            ex: Undefined,
            _marker: PhantomData,
        }
    }
}

impl<Entity: EntityTrait> MapCase<Entity, Undefined, Undefined> {
    pub fn with_fields<F: IntoIterator<Item = Entity::Attr>>(
        self,
        fields: F,
    ) -> MapCase<Entity, F, Undefined> {
        MapCase {
            kind: self.kind,
            fields,
            ex: self.ex,
            _marker: PhantomData,
        }
    }
}

impl<Entity, Fields> MapCase<Entity, Fields, Undefined> {
    pub fn then_throw<E>(self, ex: E) -> MapCase<Entity, Fields, E> {
        MapCase {
            kind: self.kind,
            fields: self.fields,
            ex,
            _marker: PhantomData,
        }
    }
}
