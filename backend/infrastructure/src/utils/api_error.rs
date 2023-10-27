use axum::response::{IntoResponse, Response};
use http::{HeaderMap, StatusCode};

use super::ResBody;

const INTERNAL_ERROR_MSG: &str = "internal server error";

pub trait IntoApiError: std::fmt::Debug + std::fmt::Display + Sized {
    fn status_code(&self) -> StatusCode;

    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    #[tracing::instrument]
    fn into_api_error(self) -> ApiError {
        let status_code = self.status_code();
        let headers = self.headers();

        let message = match status_code {
            StatusCode::INTERNAL_SERVER_ERROR => {
                tracing::error!(details=%self, "an internal server error occured");
                INTERNAL_ERROR_MSG.to_owned()
            }
            _ => self.to_string(),
        };

        ApiError {
            status_code,
            headers,
            body: ResBody {
                message,
                data: None,
            },
        }
    }
}

impl IntoApiError for anyhow::Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub body: ResBody<String, ()>,
    pub headers: HeaderMap,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body.message)
    }
}

impl std::error::Error for ApiError {}

impl<I: IntoApiError> From<I> for ApiError {
    fn from(value: I) -> Self {
        value.into_api_error()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code, self.headers, self.body).into_response()
    }
}
