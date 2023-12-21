use utils::di::{Module, Provide};

// pub mod attestation;
// pub mod auth_service;
// pub mod class;
pub mod curriculum;
// pub mod curriculum_module;
// pub mod discipline;
pub mod passport;
pub mod person;

// pub mod student;
// pub mod study_group;
pub mod subdivision;
// pub mod teacher;
// pub mod token;
pub mod tag;
pub mod university;
pub mod user;
pub mod user_session;

pub trait AdaptersModule: Send + Module
// + Provide<user::BoxedRepo>
// + Provide<user::BoxedPasswordHasher>
// + Provide<user_session::BoxedRepo>
// + Provide<user_session::SessionTTL>
// + Provide<user_session::SessionsMaxNumber>
// + Provide<university::BoxedRepo>
// + Provide<subdivision::BoxedRepo>
// + Provide<tag::BoxedRepo>
// + Provide<passport::BoxedRepo>
// + Provide<person::BoxedRepo>
// + Provide<token::BoxedAccessTokenEngine>
// + Provide<token::BoxedRefreshTokenGenerator>
// + Provide<token::AccessTokenTTL>
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
