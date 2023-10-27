use std::marker::PhantomData;

use anyhow::Context;
use app::token::{Claims, ExtractClaimsError, TokenService};
use axum::extract::FromRequestParts;
use http::header::WWW_AUTHENTICATE;
use http::request::Parts;
use http::{header, HeaderMap, HeaderValue, StatusCode};

use crate::utils::RoleChecker;
use crate::utils::{
    api_error::{ApiError, IntoApiError},
    CommonState,
};
use di::Module;

use super::DiContainer;

pub struct JwtClaims<C: RoleChecker>(pub Claims, PhantomData<C>);

#[derive(Debug, thiserror::Error)]
pub enum ExtractJwtClaimsError {
    #[error("no rights")]
    NoRights,
    #[error("missing authorization header")]
    MissingHeader,
    #[error("unsupported scheme")]
    UnsupportedScheme,
    #[error(transparent)]
    ExtractClaimsError(#[from] ExtractClaimsError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoApiError for ExtractClaimsError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::ExpiredToken => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoApiError for ExtractJwtClaimsError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingHeader => StatusCode::UNAUTHORIZED,
            Self::UnsupportedScheme => StatusCode::UNAUTHORIZED,
            Self::NoRights => StatusCode::UNAUTHORIZED,
            Self::ExtractClaimsError(err) => err.status_code(),
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn headers(&self) -> http::HeaderMap {
        let mut headers = HeaderMap::new();

        match self {
            Self::ExtractClaimsError(err) => headers.extend(err.headers().into_iter()),
            Self::MissingHeader => {
                headers.insert(WWW_AUTHENTICATE, HeaderValue::from_str("Bearer").unwrap());
            }
            _ => (),
        }

        headers
    }
}

#[async_trait::async_trait]
impl<C: RoleChecker, S: CommonState> FromRequestParts<S> for JwtClaims<C> {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let DiContainer(di) = DiContainer::from_request_parts(parts, state)
            .await
            .context("failed to extract DiContainer")?;

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or(ExtractJwtClaimsError::MissingHeader)?
            .to_str()
            .context("failed to extract authorization header as str")?;

        if !auth_header.starts_with("Bearer ") {
            return Err(ExtractJwtClaimsError::UnsupportedScheme.into());
        }

        let bearer_token = &auth_header[7..];

        let claims = di
            .resolve::<TokenService>()
            .extract_claims(bearer_token)
            .await?;

        if !C::can_access(claims.role) {
            return Err(ExtractJwtClaimsError::NoRights.into());
        }

        Ok(JwtClaims(claims, PhantomData))
    }
}
