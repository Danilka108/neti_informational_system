use utils::di::{Module, Provide};

pub mod attestation;
pub mod auth_service;
pub mod class;
pub mod class_kind;
pub mod curriculum;
pub mod curriculum_module;
pub mod discipline;
pub mod hasher;
pub mod passport;
pub mod person;
pub mod person_service;
pub mod student;
pub mod study_group;
pub mod subdivision;
pub mod tag;
pub mod teacher;
pub mod token;
pub mod university;
pub mod user;
pub mod user_service;
pub mod user_session;

pub trait AdaptersModule:
    Send
    + Module
    + Provide<user::BoxedRepo>
    + Provide<hasher::BoxedPasswordHasher>
    + Provide<user_session::BoxedRepo>
    + Provide<user_session::SessionTTL>
    + Provide<user_session::SessionsMaxNumber>
    + Provide<university::BoxedRepo>
    + Provide<subdivision::BoxedRepo>
    + Provide<tag::BoxedRepo>
    + Provide<passport::BoxedRepo>
    + Provide<person::BoxedRepo>
    + Provide<token::BoxedAccessTokenEngine>
    + Provide<token::BoxedRefreshTokenGenerator>
    + Provide<token::AccessTokenTTL>
    + Provide<study_group::BoxedRepo>
    + Provide<student::BoxedRepo>
    + Provide<discipline::BoxedRepo>
    + Provide<curriculum::BoxedRepo>
    + Provide<curriculum_module::BoxedRepo>
    + Provide<class_kind::BoxedRepo>
    + Provide<class::BoxedRepo>
    + Provide<attestation::BoxedRepo>
{
}

#[derive(Debug, Clone)]
pub struct AppModule<A> {
    pub adapters: A,
}

impl<A> AppModule<A> {
    pub fn new(adapters: A) -> Self {
        Self { adapters }
    }
}

impl<A> Module for AppModule<A> {}
