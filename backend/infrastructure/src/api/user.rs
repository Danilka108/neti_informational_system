use std::sync::Arc;

use anyhow::Context;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use domain::Role;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use super::common::{IntoApiError, ResBody, Result};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", post(create))
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct CreateResult {
    id: i32,
}

impl IntoApiError for app::user::CreateError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
        }
    }
}

#[axum::debug_handler]
async fn create(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreatePayload>,
) -> Result<impl IntoResponse> {
    let mut tx = app_state
        .pg_pool()
        .begin()
        .await
        .context("failed to begin postgres transaction")?;

    let user = app_state
        .user_service()
        .create(&mut tx, payload.email, Role::Admin, payload.password)
        .await?;

    tx.commit()
        .await
        .context("failed to commit postgres transaction")?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "user created successfully",
            data: Some(CreateResult { id: user.id }),
        },
    ))
}
