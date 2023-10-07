mod api_error;
mod auth;
mod res_body;
mod role_checker;
mod session_metadata;

pub use api_error::{ApiError, IntoApiError};
pub use auth::Auth;
pub use res_body::ResBody;
pub use role_checker::Admin;
pub use session_metadata::SessionMetadata;

pub type Result<R> = std::result::Result<R, ApiError>;
