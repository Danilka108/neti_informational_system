mod api_error;
pub mod extractors;
mod provide_di_container;
mod res_body;
mod role_checkers;

pub use api_error::{ApiError, IntoApiError};
pub use provide_di_container::provide_di_container;
pub use res_body::ResBody;
pub use role_checkers::{Admin, RoleChecker};

pub type Result<R> = std::result::Result<R, ApiError>;

pub trait CommonState: Clone + std::fmt::Debug + Send + Sync + 'static {}
impl<T: Clone + std::fmt::Debug + Send + Sync + 'static> CommonState for T {}
