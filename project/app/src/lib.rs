#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(exhaustive_patterns)]

use std::{fmt::Display, marker::PhantomData, ops::Deref};

// use auth::AuthService;
use di::{Module, Provide};
// use person::{DynPersonRepository, PersonService};
// use ports::{
//     AccessTokenTTL, BoxedSubdivisionMemberRepository, BoxedSubdivisionRepository,
//     BoxedSubdivisionTagRepository, BoxedTagRepository, BoxedUniversityRepository,
//     DynAccessTokenEngine, DynPasswordHasher, DynRefreshTokenGenerator, DynSessionRepository,
//     DynUserRepository, SessionTTL, SessionsMaxNumber,
// };

// use session::SessionService;
// use tag::TagService;
// use token::TokenService;
// use user::UserService;

// pub mod auth;
pub mod outcome;

pub mod person;
pub mod subdivision;
pub mod university;

mod base_repo;
// pub mod person;
// pub mod ports;
// pub mod session;
// pub mod subdivision;
// pub mod tag;
// pub mod token;
// pub mod university;
// pub mod user;

pub use outcome::Outcome;

pub trait EntityTrait {
    const NAME: &'static str;

    type Field: FieldTrait;
    type IdValue: Display;

    fn get_field_value(&self, field: Self::Field) -> String;
}

pub trait FieldTrait {
    fn name(&self) -> &'static str;
}

#[derive(Debug)]
pub struct Id<E: EntityTrait> {
    pub value: E::IdValue,
    _marker: std::marker::PhantomData<E>,
}

impl<E: EntityTrait> Clone for Id<E>
where
    E::IdValue: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<E: EntityTrait> Copy for Id<E> where E::IdValue: Copy {}

impl<E: EntityTrait> Display for Id<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
// pub struct Ref<I, E>(pub I, PhantomData<E>);

// impl<I: Default, E> Default for Ref<I, E> {
//     fn default() -> Self {
//         Self(I::default(), PhantomData)
//     }
// }

// impl<I, E> From<I> for Ref<I, E> {
//     fn from(value: I) -> Self {
//         Self(value, PhantomData)
//     }
// }

// impl<I, E> Deref for Ref<I, E> {
//     type Target = I;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// pub type SerialId = i32;

pub trait AdaptersModule:
    Module
    + Provide<person::BoxedRepo>
    + Provide<university::BoxedRepo>
    + Provide<subdivision::BoxedRepo>
// + Provide<SessionTTL>
// + Provide<SessionsMaxNumber>
// + Provide<DynSessionRepository>
// + Provide<DynPasswordHasher>
// + Provide<DynUserRepository>
// + Provide<AccessTokenTTL>
// + Provide<DynAccessTokenEngine>
// + Provide<DynRefreshTokenGenerator>
// + Provide<DynPersonRepository>
// + Provide<BoxedUniversityRepository>
// + Provide<BoxedSubdivisionRepository>
// + Provide<BoxedSubdivisionTagRepository>
// + Provide<BoxedSubdivisionMemberRepository>
// + Provide<BoxedTagRepository>
{
}

#[derive(Debug, Clone)]
pub struct AppModule<A> {
    pub(crate) adapters: A,
}

impl<A> AppModule<A> {
    pub fn new(adapters: A) -> Self {
        Self { adapters }
    }
}

impl<A> Module for AppModule<A> {}

// impl<A: AdaptersModule> Provide<UserService> for AppModule<A> {
//     fn provide(&self) -> UserService {
//         UserService {
//             repo: self.adapters.resolve(),
//             pass_hasher: self.adapters.resolve(),
//             person_service: self.resolve(),
//         }
//     }
// }

// impl<A: AdaptersModule> Provide<SessionService> for AppModule<A> {
//     fn provide(&self) -> SessionService {
//         SessionService {
//             repo: self.adapters.resolve(),
//             sessions_max_number: self.adapters.resolve(),
//             session_ttl: self.adapters.resolve(),
//         }
//     }
// }

// impl<A: AdaptersModule> Provide<TokenService> for AppModule<A> {
//     fn provide(&self) -> TokenService {
//         TokenService {
//             access_token_engine: self.adapters.resolve(),
//             access_token_ttl: self.adapters.resolve(),
//             refresh_token_generator: self.adapters.resolve(),
//         }
//     }
// }

// impl<A: AdaptersModule> Provide<PersonService> for AppModule<A> {
//     fn provide(&self) -> PersonService {
//         PersonService {
//             repo: self.adapters.resolve(),
//         }
//     }
// }

// impl<A: AdaptersModule> Provide<AuthService> for AppModule<A> {
//     fn provide(&self) -> AuthService {
//         AuthService {
//             token_service: self.resolve(),
//             session_service: self.resolve(),
//             user_service: self.resolve(),
//         }
//     }
// }

// impl<A: AdaptersModule> Provide<TagService> for AppModule<A> {
//     fn provide(&self) -> TagService {
//         TagService {
//             repo: self.adapters.provide(),
//         }
//     }
// }
