use app::auth::{AuthService, LoginException, LogoutException, RefreshTokenException};
use axum::{response::IntoResponse, routing::post, Json, Router};
use di::Module;
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::utils::{
    extractors::{DiContainer, SessionMetadata},
    ApiResult, EmptyData, Reply,
};

use crate::utils::CommonState;

pub fn router<S: CommonState>() -> Router<S> {
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

struct LoginError(LoginException);

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        let Self(ex) = self;

        let code = match ex {
            LoginException::FailedToSaveSession(_) | LoginException::FailedToAuthenticate(_) => {
                StatusCode::UNAUTHORIZED
            }
        };

        (code, Reply::from(ex)).into_response()
    }
}

#[axum::debug_handler]
async fn login(
    DiContainer(di): DiContainer,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<LoginPayload>,
) -> ApiResult {
    let tokens = di
        .resolve::<AuthService>()
        .login(&payload.email, &payload.password, metadata)
        .await
        .map_exception(LoginError)?;

    ApiResult::new((
        StatusCode::OK,
        Reply {
            message: "login complited successfully",
            data: json!({
                "access_token": tokens.access_token,
                "refresh_token": tokens.refresh_token,
            }),
        },
    ))
}

#[derive(Debug, Deserialize)]
struct RefreshTokenPayload {
    refresh_token: String,
    user_id: i32,
}

struct RefreshTokenError(RefreshTokenException);

impl IntoResponse for RefreshTokenError {
    fn into_response(self) -> axum::response::Response {
        let Self(ex) = self;

        let code = match ex {
            RefreshTokenException::UserDoesNotExist => StatusCode::BAD_REQUEST,
            RefreshTokenException::FailedToUpdateSession(_) => StatusCode::UNAUTHORIZED,
        };

        (code, Reply::from(ex)).into_response()
    }
}

#[axum::debug_handler]
async fn refresh_token(
    DiContainer(di): DiContainer,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> ApiResult {
    let tokens = di
        .resolve::<AuthService>()
        .refresh_token(payload.user_id, &payload.refresh_token, metadata)
        .await
        .map_exception(RefreshTokenError)?;

    ApiResult::new((
        StatusCode::OK,
        Reply {
            message: "token refreshed successfully",
            data: json!({
                "access_token": tokens.access_token,
                "refresh_token": tokens.refresh_token,
            }),
        },
    ))
}

struct LogoutError(LogoutException);

impl IntoResponse for LogoutError {
    fn into_response(self) -> axum::response::Response {
        let Self(ex) = self;

        let code = match ex {
            LogoutException::UserDoesNotExist => StatusCode::BAD_REQUEST,
            LogoutException::FailedToDeleteSession(_) => StatusCode::UNAUTHORIZED,
        };

        (code, Reply::from(ex)).into_response()
    }
}

#[axum::debug_handler]
async fn logout(
    DiContainer(di): DiContainer,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> ApiResult {
    di.resolve::<AuthService>()
        .logout(payload.user_id, &payload.refresh_token, &metadata)
        .await
        .map_exception(LogoutError)?;

    ApiResult::new((
        StatusCode::OK,
        Reply {
            message: "logout is successful",
            data: EmptyData,
        },
    ))
}
