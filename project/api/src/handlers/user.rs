use app::user::{CreateUserException, Role, UserService};
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};
use di::Module;
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::utils::{extractors::DiContainer, Reply};

use crate::utils::{ApiResult, CommonState};

pub fn router<S: CommonState>() -> Router<S> {
    Router::new().route("/", post(create))
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    email: String,
    password: String,
}

struct CreateError(CreateUserException);

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        let Self(ex) = self;

        let code = match ex {
            CreateUserException::FailedToCreatePerson(_) => StatusCode::BAD_REQUEST,
            CreateUserException::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
        };

        (code, Reply::from(ex)).into_response()
    }
}

#[axum::debug_handler]
async fn create(DiContainer(di): DiContainer, Json(payload): Json<CreatePayload>) -> ApiResult {
    let user = di
        .resolve::<UserService>()
        .create(payload.email, payload.password, Role::Admin)
        .await
        .map_exception(CreateError)?;

    ApiResult::new((
        StatusCode::OK,
        Reply {
            message: "user created successfully",
            data: json!({
                "id": *user.id
            }),
        },
    ))
}
