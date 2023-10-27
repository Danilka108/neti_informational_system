use app::person::CreatePersonError;
use app::user::{CreateUserError, Role, UserService};
use axum::{response::IntoResponse, routing::post, Json, Router};
use di::Module;
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::utils::{extractors::DiContainer, IntoApiError, ResBody, Result};

use crate::utils::CommonState;

pub fn router<S: CommonState>() -> Router<S> {
    Router::new().route("/", post(create))
}

impl IntoApiError for CreatePersonError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoApiError for CreateUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CreatePersonError(err) => err.status_code(),
            Self::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
        }
    }
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

#[axum::debug_handler]
async fn create(
    DiContainer(di): DiContainer,
    Json(payload): Json<CreatePayload>,
) -> Result<impl IntoResponse> {
    let user = di
        .resolve::<UserService>()
        .create(payload.email, payload.password, Role::Admin)
        .await?;

    Ok((
        StatusCode::OK,
        ResBody {
            message: "user created successfully",
            data: Some(CreateResult { id: user.id }),
        },
    ))
}
