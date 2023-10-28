use app::AppModule;
use axum::extract::FromRequestParts;
use http::{request::Parts, StatusCode};

use crate::adapters::AdaptersModule;
use crate::config::ConfigContainer;

use crate::utils::CommonState;
use crate::utils::{ApiError, IntoApiError};

#[derive(Debug, Clone)]
pub struct DiContainer(pub AppModule<AdaptersModule<ConfigContainer>>);

#[derive(Debug, thiserror::Error)]
#[error("DiContainer is missing in request extensions")]
struct ExtractDiContainerError;

impl IntoApiError for ExtractDiContainerError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[async_trait::async_trait]
impl<S: CommonState> FromRequestParts<S> for DiContainer {
    type Rejection = ApiError;

    #[tracing::instrument(name = "Build di container", skip(parts, _state))]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(di_contaner) = parts.extensions.get::<DiContainer>().cloned() else {
            tracing::error!("failed to get DiContainer extension");
            return Err(ExtractDiContainerError.into());
        };

        Ok(di_contaner)
    }
}
