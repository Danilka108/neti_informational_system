use crate::state::AppState;

use app::token::ValidateAccessTokenError;
use axum::extract::FromRequestParts;
use domain::AuthClaims;
use http::header::WWW_AUTHENTICATE;
use http::request::Parts;
use http::{header, HeaderMap, StatusCode};

use super::api_error::ApiError;
use super::role_checker::RoleChecker;
use super::IntoApiError;

pub struct Auth<C: RoleChecker> {
    pub claims: AuthClaims,
    _checker: std::marker::PhantomData<C>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("missing authorization header")]
    MissingHeader,
    #[error("unsupported scheme")]
    UnsupportedScheme,
    #[error("no rights")]
    NoRights,
    #[error(transparent)]
    ValidateTokenError(#[from] ValidateAccessTokenError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoApiError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingHeader => StatusCode::UNAUTHORIZED,
            Self::UnsupportedScheme => StatusCode::UNAUTHORIZED,
            Self::ValidateTokenError(_) => StatusCode::UNAUTHORIZED,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    fn headers(&self) -> http::HeaderMap {
        let mut headers = HeaderMap::new();

        match self {
            Self::MissingHeader => {
                headers.insert(WWW_AUTHENTICATE, "Bearer");
            }
            _ => (),
        }

        headers
    }
}

#[async_trait::async_trait]
impl<C: RoleChecker> FromRequestParts<AppState> for Auth<C> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        app_state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or(AuthError::MissingHeader)?
            .to_str()
            .map_err(|_| AuthError::InternalError)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AuthError::UnsupportedScheme);
        }

        let bearer_token = &auth_header[7..];

        let claims = app_state
            .token_service()
            .validate_access_token(bearer_token)?;

        if !C::can_access(claims.role) {
            return Err(AuthError::Forbidden);
        }

        Ok(Auth {
            claims,
            _checker: std::marker::PhantomData,
        })
    }
}
