use app::auth_service::{AuthException, AuthService};
use axum::{response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use utils::di::Module;

use crate::utils::{
    extractors::{ReqScopeModule, SessionMetadata},
    ApiResult, EmptyData, Reply,
};

use crate::utils::CommonState;

use super::user;

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

struct Exception(AuthException);

impl IntoResponse for Exception {
    #[allow(unreachable_code)]
    fn into_response(self) -> axum::response::Response {
        let Self(ex) = self;
        let code: StatusCode = match ex {
            AuthException::UserException(ex) => return user::Exception(ex).into_response(),
        };

        (code, Reply::from(ex)).into_response()
    }
}

#[axum::debug_handler]
async fn login(
    ReqScopeModule(module): ReqScopeModule,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<LoginPayload>,
) -> ApiResult {
    let tokens = module
        .resolve::<AuthService>()
        .login(payload.email, payload.password, metadata)
        .await
        .map_ex(Exception)?;

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

#[axum::debug_handler]
async fn refresh_token(
    ReqScopeModule(module): ReqScopeModule,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> ApiResult {
    let tokens = module
        .resolve::<AuthService>()
        .refresh_token(payload.user_id, payload.refresh_token, metadata)
        .await
        .map_ex(Exception)?;

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

#[axum::debug_handler]
async fn logout(
    ReqScopeModule(module): ReqScopeModule,
    SessionMetadata(metadata): SessionMetadata,
    Json(payload): Json<RefreshTokenPayload>,
) -> ApiResult {
    module
        .resolve::<AuthService>()
        .logout(payload.user_id, payload.refresh_token, metadata)
        .await
        .map_ex(Exception)?;

    ApiResult::new((
        StatusCode::OK,
        Reply {
            message: "logout is successful",
            data: EmptyData,
        },
    ))
}
