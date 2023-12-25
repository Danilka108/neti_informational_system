use app::user_service::{UserException, UserService};
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use utils::di::Module;

use crate::utils::{extractors::ReqScopeModule, Reply};

use crate::utils::{ApiResult, CommonState};

pub fn router<S: CommonState>() -> Router<S> {
    Router::new().route("/", post(create))
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    email: String,
    password: String,
}

#[derive(Debug)]
pub struct Exception(pub UserException);

impl IntoResponse for Exception {
    fn into_response(self) -> axum::response::Response {
        let Self(ex) = self;
        let ex = dbg!(ex);

        // tracing::info!(?ex, "exception was thrown");
        let code = match ex {
            UserException::UserNotFound => StatusCode::BAD_REQUEST,
            UserException::EmailAlreadyInUseOrInvalidPassport => StatusCode::UNAUTHORIZED,
            UserException::SessionExpired => StatusCode::UNAUTHORIZED,
            UserException::SessionNotFound => StatusCode::UNAUTHORIZED,
            UserException::InvalidRefreshToken => StatusCode::UNAUTHORIZED,
            UserException::SessionsLimitReached => StatusCode::UNAUTHORIZED,
        };

        (code, Reply::from(ex)).into_response()
    }
}

#[axum::debug_handler]
async fn create(
    ReqScopeModule(module): ReqScopeModule,
    Json(payload): Json<CreatePayload>,
) -> ApiResult {
    let user = module
        .resolve::<UserService>()
        .create(payload.email, payload.password)
        .await
        .map_ex(Exception);
    let user = dbg!(user)?;

    ApiResult::new((
        StatusCode::OK,
        Reply {
            message: "user created successfully",
            data: json!({
                "id": user.id.value
            }),
        },
    ))
}
