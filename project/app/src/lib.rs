use utils::di::{Module, Provide};

pub mod person;
pub mod subdivision;
pub mod tag;
pub mod university;

pub trait AdaptersModule:
    Send
    + Module
    + Provide<person::BoxedRepo>
    + Provide<person::BoxedPasswordHasher>
    + Provide<university::BoxedRepo>
    + Provide<subdivision::BoxedRepo>
    + Provide<tag::BoxedRepo>
// + Provide<SessionTTL>
// + Provide<SessionsMaxNumber>
// + Provide<DynSessionRepository>
// + Provide<DynPasswordHasher>
// + Provide<AccessTokenTTL>
// + Provide<DynAccessTokenEngine>
// + Provide<DynRefreshTokenGenerator>
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
