use log::LevelFilter;
use std::sync::Arc;

pub use crate::adapters::password::Argon2Params;
pub use crate::adapters::tokens::{JwtKeys, RefreshTokenLength};
pub use app::ports::AccessTokenTTL;
pub use app::ports::SessionTTL;
pub use app::ports::SessionsMaxNumber;
use di::{Module, Provide};

pub struct PgHost(pub Arc<str>);
pub struct PgPort(pub u16);
pub struct PgUserName(pub Arc<str>);
pub struct PgPassword(pub Arc<str>);
pub struct PgDatabaseName(pub Arc<str>);
pub struct ApplicationName(pub Arc<str>);
pub struct SqlxLogLevelFilter(pub Option<LevelFilter>);
pub struct SqlxMaxConnections(pub u32);

pub trait ConfigModule:
    Module
    + Provide<Arc<JwtKeys>>
    + Provide<RefreshTokenLength>
    + Provide<Argon2Params>
    + Provide<AccessTokenTTL>
    + Provide<SessionTTL>
    + Provide<SessionsMaxNumber>
    + Provide<PgHost>
    + Provide<PgPort>
    + Provide<PgUserName>
    + Provide<PgPassword>
    + Provide<PgDatabaseName>
    + Provide<ApplicationName>
    + Provide<SqlxLogLevelFilter>
    + Provide<SqlxMaxConnections>
{
}
