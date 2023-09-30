use axum::response::{IntoResponse, Response};
use http::header::WWW_AUTHENTICATE;
use http::StatusCode;

#[derive(Debug)]
pub enum AuthError {
    MissingHeader,
    UnsupportedScheme,
    ExpiredToken,
    InvalidToken,
    InternalError,
    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::MissingHeader => {
                (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response()
            }
            Self::UnsupportedScheme => (
                StatusCode::UNAUTHORIZED,
                [(WWW_AUTHENTICATE, "Bearer")],
                "Unsupported authentication scheme",
            )
                .into_response(),
            Self::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token").into_response(),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
            Self::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
            }
            Self::Forbidden => (StatusCode::FORBIDDEN, "No rights").into_response(),
        }
    }
}
