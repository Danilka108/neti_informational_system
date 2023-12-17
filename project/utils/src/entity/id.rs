use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::EntityTrait;

#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<Entity: EntityTrait> {
    pub value: Entity::IdValue,
    _marker: std::marker::PhantomData<Entity>,
}

pub trait ProvideId<Entity: EntityTrait> {
    fn provide_id(&self) -> &Id<Entity>;
}

impl<Entity: EntityTrait> ProvideId<Entity> for Entity::IdValue {
    fn provide_id(&self) -> &Id<Entity> {
        unsafe { &*(self as *const Entity::IdValue as *const Id<Entity>) }
    }
}

impl<Entity: EntityTrait> PartialEq for Id<Entity>
where
    Entity::IdValue: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }

    fn ne(&self, other: &Self) -> bool {
        self.value.ne(&other.value)
    }
}

impl<Entity: EntityTrait> Eq for Id<Entity> where Entity::IdValue: Eq {}

impl<Entity: EntityTrait> Hash for Id<Entity>
where
    Entity::IdValue: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<Entity> Default for Id<Entity>
where
    Entity: EntityTrait,
    Entity::IdValue: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<Entity: EntityTrait> Clone for Id<Entity>
where
    Entity::IdValue: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<Entity: EntityTrait> Copy for Id<Entity> where Entity::IdValue: Copy {}

impl<Entity: EntityTrait> std::fmt::Display for Id<Entity>
where
    Entity::IdValue: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<Entity> std::fmt::Debug for Id<Entity>
where
    Entity: EntityTrait,
    Entity::IdValue: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id").field("value", &self.value).finish()
    }
}

impl<E: EntityTrait> Id<E> {
    pub fn new(value: E::IdValue) -> Id<E> {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}
