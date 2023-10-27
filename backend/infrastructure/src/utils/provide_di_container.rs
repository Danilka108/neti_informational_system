use std::sync::Arc;

use axum::{extract::State, middleware::Next, response::Response};
use http::{Request, StatusCode};

use crate::state::AppState;

use super::extractors::DiContainer;

#[tracing::instrument]
pub async fn provide_di_container<B: std::fmt::Debug>(
    State(app_state): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let (di_container, txn) = match app_state.begin().await {
        Ok((app_module, txn)) => (DiContainer(app_module), txn),
        Err(err) => {
            tracing::error!(%err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let None = request.extensions_mut().insert(di_container) else {
        tracing::error!("DiContainer extension already exist");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let response = next.run(request).await;

    let Some(txn) = Arc::into_inner(txn) else {
        tracing::error!(
            "failed to extract transaction, someone still has a reference to the transaction"
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if let Err(err) = txn.into_inner().commit().await {
        tracing::error!(%err,
            "failed to commit transaction"
        );

        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(response)
}
