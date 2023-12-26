use std::{convert::Infallible, ops::FromResidual};

use axum::response::{IntoResponse, Response};
use http::StatusCode;
use utils::outcome::Outcome;

use super::{EmptyData, Reply};

const INTERNAL_ERROR_MSG: &str = "internal server error";

pub struct ApiResult(Response);

impl ApiResult {
    pub fn new(response: impl IntoResponse) -> Self {
        Self(response.into_response())
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn anyhow_error_into_response(error: anyhow::Error) -> Response {
    tracing::error!(?error);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Reply {
            message: INTERNAL_ERROR_MSG,
            data: EmptyData,
        },
    )
        .into_response()
}

impl IntoResponse for ApiResult {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

impl<E: IntoResponse> FromResidual<Result<Infallible, E>> for ApiResult {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(val) => ApiResult::new(val),
        }
    }
}

impl<E: IntoResponse> FromResidual<Outcome<Infallible, E>> for ApiResult {
    fn from_residual(residual: Outcome<Infallible, E>) -> Self {
        match residual {
            Outcome::Ok(_) => unreachable!(),
            Outcome::Ex(val) => ApiResult::new(val),
            Outcome::Error(val) => ApiResult(anyhow_error_into_response(val)),
        }
    }
}
