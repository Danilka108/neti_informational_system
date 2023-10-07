use std::sync::Arc;

use anyhow::Context;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use serde::Deserialize;

use super::common::{IntoApiError, ResBody, Result, SessionMetadata};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh-token", post(refresh_token))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

impl IntoApiError for app::auth::LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NewSessionError(_) => StatusCode::BAD_REQUEST,
            Self::AuthenticateError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[axum::debug_handler]
async fn login(
    State(app_state): State<Arc<AppState>>,
    SessionMetadata(session_metadata): SessionMetadata,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse> {
    let mut tx = app_state
        .pg_pool()
        .begin()
        .await
        .context("failed to begin postgres transaction")?;

    let tokens_pair = app_state
        .auth_service()
        .login(&mut tx, &payload.email, &payload.password, session_metadata)
        .await?;

    tx.commit()
        .await
        .context("failed to commit postgres transaction")?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "login complited successfully",
            data: Some(tokens_pair),
        },
    ))
}

#[derive(Debug, Deserialize)]
struct RefreshTokenPayload {
    refresh_token: String,
    user_id: i32,
}

impl IntoApiError for app::auth::RefreshTokenError {
    #[tracing::instrument]
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserDoesNotExist | Self::NewSessionError(_) | Self::ValidateSessionError(_) => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}

#[axum::debug_handler]
async fn refresh_token(
    State(app_state): State<Arc<AppState>>,
    SessionMetadata(session_metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<impl IntoResponse> {
    let mut tx = app_state
        .pg_pool()
        .begin()
        .await
        .context("failed to begin postgres transaction")?;

    let tokens_pair = app_state
        .auth_service()
        .refresh_token(
            &mut tx,
            payload.user_id,
            &payload.refresh_token,
            session_metadata,
        )
        .await?;

    tx.commit()
        .await
        .context("failed to commit postgres transaction")?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "token refreshed successfully",
            data: Some(tokens_pair),
        },
    ))
}

impl IntoApiError for app::auth::LogoutError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserDoesNotExist
            | Self::ValidateSessionError(_)
            | Self::DeleteSessionError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[axum::debug_handler]
async fn logout(
    State(app_state): State<Arc<AppState>>,
    SessionMetadata(session_metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<impl IntoResponse> {
    let mut tx = app_state
        .pg_pool()
        .begin()
        .await
        .context("failed to begin postgres transaction")?;

    app_state
        .auth_service()
        .logout(
            &mut tx,
            payload.user_id,
            &payload.refresh_token,
            &session_metadata,
        )
        .await?;

    tx.commit()
        .await
        .context("failed to commit postgres transaction")?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "logout is successful",
            data: None::<()>,
        },
    ))
}
