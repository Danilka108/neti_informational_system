use adapters::transaction::TransactionModule;
use app::AppModule;
use axum::{extract::FromRequestParts, response::IntoResponse};
use http::{request::Parts, StatusCode};

use crate::utils::{CommonState, Reply};

#[derive(Debug, Clone)]
pub struct ReqScopeModule(pub AppModule<TransactionModule<crate::config::ConfigModule>>);

#[derive(Debug, thiserror::Error)]
#[error("TxnScope is missing in request extensions")]
pub struct ExtractTxnScopeError;

impl IntoResponse for ExtractTxnScopeError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Reply::from(self)).into_response()
    }
}

#[async_trait::async_trait]
impl<S: CommonState> FromRequestParts<S> for ReqScopeModule {
    type Rejection = ExtractTxnScopeError;

    #[tracing::instrument(name = "Build di container", skip(parts, _state))]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(di_contaner) = parts.extensions.get::<ReqScopeModule>().cloned() else {
            tracing::error!("failed to get DiContainer extension");
            return Err(ExtractTxnScopeError.into());
        };

        Ok(di_contaner)
    }
}
