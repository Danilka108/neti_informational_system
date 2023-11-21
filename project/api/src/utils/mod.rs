mod api_result;
pub mod extractors;
mod provide_di_container;
mod reply;
mod role_checkers;

pub use api_result::ApiResult;
pub use provide_di_container::provide_di_container;
pub use reply::{EmptyData, Reply};
pub use role_checkers::{Admin, RoleChecker};

pub trait CommonState: Clone + std::fmt::Debug + Send + Sync + 'static {}
impl<T: Clone + std::fmt::Debug + Send + Sync + 'static> CommonState for T {}
