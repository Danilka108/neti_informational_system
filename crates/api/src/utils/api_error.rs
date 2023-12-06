use axum::{extract::rejection::JsonRejection, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            Self::JsonExtractorRejection(rej) => (rej.status(), rej.body_text()),
        };

        (status, super::res::error(msg)).into_response()
    }
}
