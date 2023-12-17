use utils::di::{Module, Provide};

mod auth_service;
pub mod passport;
pub mod person;
pub mod subdivision;
pub mod tag;
pub mod token;
pub mod university;
pub mod user;

pub trait AdaptersModule:
    Send
    + Module
    + Provide<user::BoxedRepo>
    + Provide<user::BoxedPasswordHasher>
    + Provide<user::SessionTTL>
    + Provide<user::SessionsMaxNumber>
    + Provide<university::BoxedRepo>
    + Provide<subdivision::BoxedRepo>
    + Provide<tag::BoxedRepo>
    + Provide<passport::BoxedRepo>
    + Provide<person::BoxedRepo>
    + Provide<token::BoxedAccessTokenEngine>
    + Provide<token::BoxedRefreshTokenGenerator>
    + Provide<token::AccessTokenTTL>
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
