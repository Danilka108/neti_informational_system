use app::AppModule;
use axum::{extract::State, middleware::Next, response::Response};
use http::{Request, StatusCode};

use crate::api_state::ApiState;

use super::extractors::ReqScopeModule;

#[tracing::instrument(skip(request, next))]
pub async fn provide_req_scope_module<B>(
    State(app_state): State<ApiState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let adapters = match app_state.begin_request_scope().await {
        Ok(val) => val,
        Err(err) => {
            tracing::error!(%err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let req_scope_module = ReqScopeModule(AppModule::new(adapters.clone()));
    let None = request.extensions_mut().insert(req_scope_module) else {
        tracing::error!("DiContainer extension already exist");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let response = next.run(request).await;

    match adapters.commit().await {
        Ok(()) => Ok(response),
        Err(err) => {
            tracing::error!(%err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
