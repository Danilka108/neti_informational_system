use app::auth::AuthService;
use axum::{response::IntoResponse, routing::post, Json, Router};
use di::Module;
use http::StatusCode;
use serde::Deserialize;

use crate::utils::{
    extractors::{DiContainer, SessionMetadata},
    IntoApiError, ResBody, Result,
};

use crate::utils::CommonState;

pub fn router<S: CommonState>() -> Router<S> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh-token", post(refresh_token))
}

impl IntoApiError for app::session::SaveSessionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SessionsLimitReached { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoApiError for app::session::ValidateSessionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoSessionFound | Self::SessionExpired | Self::InvalidRefreshToken => {
                StatusCode::UNAUTHORIZED
            }
        }
    }
}

impl IntoApiError for app::session::UpdateSessionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidateSessionError(err) => err.status_code(),
        }
    }
}

impl IntoApiError for app::user::AuthenticateUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidLoginOrPassword => StatusCode::UNAUTHORIZED,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoApiError for app::auth::LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SaveSessionError(err) => err.status_code(),
            Self::AuthenticateError(err) => err.status_code(),
        }
    }
}

impl IntoApiError for app::auth::RefreshTokenError {
    #[tracing::instrument]
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserDoesNotExist => StatusCode::BAD_REQUEST,
            Self::UpdateSessionError(err) => err.status_code(),
        }
    }
}

impl IntoApiError for app::session::DeleteSessionError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidateError(err) => err.status_code(),
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoApiError for app::auth::LogoutError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserDoesNotExist => StatusCode::BAD_REQUEST,
            Self::DeleteSessionError(err) => err.status_code(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct RefreshTokenPayload {
    refresh_token: String,
    user_id: i32,
}

#[axum::debug_handler]
async fn login(
    DiContainer(di): DiContainer,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse> {
    let tokens = di
        .resolve::<AuthService>()
        .login(&payload.email, &payload.password, metadata)
        .await?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "login complited successfully",
            data: Some(tokens),
        },
    ))
}

#[axum::debug_handler]
async fn refresh_token(
    DiContainer(di): DiContainer,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<impl IntoResponse> {
    let tokens = di
        .resolve::<AuthService>()
        .refresh_token(payload.user_id, &payload.refresh_token, metadata)
        .await?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "token refreshed successfully",
            data: Some(tokens),
        },
    ))
}

#[axum::debug_handler]
async fn logout(
    DiContainer(di): DiContainer,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> Result<impl IntoResponse> {
    let tokens = di
        .resolve::<AuthService>()
        .logout(payload.user_id, &payload.refresh_token, &metadata)
        .await?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "logout is successful",
            data: Some(tokens),
        },
    ))
}
