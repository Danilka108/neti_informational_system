#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(exhaustive_patterns)]

use std::{marker::PhantomData, ops::Deref};

use auth::AuthService;
use di::{Module, Provide};
use person::{DynPersonRepository, PersonService};
use ports::{
    AccessTokenTTL, BoxedSubdivisionMemberRepository, BoxedSubdivisionRepository,
    BoxedSubdivisionTagRepository, BoxedTagRepository, BoxedUniversityRepository,
    DynAccessTokenEngine, DynPasswordHasher, DynRefreshTokenGenerator, DynSessionRepository,
    DynUserRepository, SessionTTL, SessionsMaxNumber,
};

use session::SessionService;
use tag::TagService;
use token::TokenService;
use user::UserService;

pub mod auth;
pub mod outcome;
pub mod person;
pub mod ports;
pub mod session;
pub mod subdivision;
pub mod tag;
pub mod token;
pub mod university;
pub mod user;

pub use outcome::Outcome;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Ref<I, E>(pub I, PhantomData<E>);

impl<I: Default, E> Default for Ref<I, E> {
    fn default() -> Self {
        Self(I::default(), PhantomData)
    }
}

impl<I, E> From<I> for Ref<I, E> {
    fn from(value: I) -> Self {
        Self(value, PhantomData)
    }
}

impl<I, E> Deref for Ref<I, E> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type SerialId = i32;

pub trait AdaptersModule:
    Module
    + Provide<SessionTTL>
    + Provide<SessionsMaxNumber>
    + Provide<DynSessionRepository>
    + Provide<DynPasswordHasher>
    + Provide<DynUserRepository>
    + Provide<AccessTokenTTL>
    + Provide<DynAccessTokenEngine>
    + Provide<DynRefreshTokenGenerator>
    + Provide<DynPersonRepository>
    + Provide<BoxedUniversityRepository>
    + Provide<BoxedSubdivisionRepository>
    + Provide<BoxedSubdivisionTagRepository>
    + Provide<BoxedSubdivisionMemberRepository>
    + Provide<BoxedTagRepository>
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

impl<A: AdaptersModule> Provide<UserService> for AppModule<A> {
    fn provide(&self) -> UserService {
        UserService {
            repo: self.adapters.resolve(),
            pass_hasher: self.adapters.resolve(),
            person_service: self.resolve(),
        }
    }
}

impl<A: AdaptersModule> Provide<SessionService> for AppModule<A> {
    fn provide(&self) -> SessionService {
        SessionService {
            repo: self.adapters.resolve(),
            sessions_max_number: self.adapters.resolve(),
            session_ttl: self.adapters.resolve(),
        }
    }
}

impl<A: AdaptersModule> Provide<TokenService> for AppModule<A> {
    fn provide(&self) -> TokenService {
        TokenService {
            access_token_engine: self.adapters.resolve(),
            access_token_ttl: self.adapters.resolve(),
            refresh_token_generator: self.adapters.resolve(),
        }
    }
}

impl<A: AdaptersModule> Provide<PersonService> for AppModule<A> {
    fn provide(&self) -> PersonService {
        PersonService {
            repo: self.adapters.resolve(),
        }
    }
}

impl<A: AdaptersModule> Provide<AuthService> for AppModule<A> {
    fn provide(&self) -> AuthService {
        AuthService {
            token_service: self.resolve(),
            session_service: self.resolve(),
            user_service: self.resolve(),
        }
    }
}

impl<A: AdaptersModule> Provide<TagService> for AppModule<A> {
    fn provide(&self) -> TagService {
        TagService {
            repo: self.adapters.provide(),
        }
    }
}
