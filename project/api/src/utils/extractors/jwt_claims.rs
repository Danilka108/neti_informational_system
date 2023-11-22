use std::marker::PhantomData;

use anyhow::Context;
use app::token::{Claims, ExtractClaimsException, TokenService};
use app::Outcome;
use axum::extract::FromRequestParts;
use axum::response::{IntoResponse, Response};
use http::request::Parts;
use http::{header, StatusCode};

use crate::utils::api_result::anyhow_error_into_response;
use crate::utils::RoleChecker;
use crate::utils::{CommonState, Reply};
use di::Module;

use super::ReqScopeModule;

pub struct Auth<C: RoleChecker>(pub Claims, PhantomData<C>);

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum AuthException {
    #[error("no rights")]
    NoRights,
    #[error("missing authorization header")]
    MissingHeader,
    #[error("unsupported scheme")]
    UnsupportedScheme,
    #[error(transparent)]
    FailedToExtractJwtClaims(#[from] ExtractClaimsException),
}

impl IntoResponse for AuthException {
    fn into_response(self) -> axum::response::Response {
        let response = (StatusCode::UNAUTHORIZED, Reply::from(self));

        match self {
            Self::MissingHeader => {
                ([(header::WWW_AUTHENTICATE, "Bearer")], response).into_response()
            }
            _ => response.into_response(),
        }
    }
}

#[async_trait::async_trait]
impl<C: RoleChecker, S: CommonState> FromRequestParts<S> for Auth<C> {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_jwt_claims(parts, state).await {
            Outcome::Success(val) => Ok(val),
            Outcome::Exception(val) => Err(val.into_response()),
            Outcome::Unexpected(val) => Err(anyhow_error_into_response(val)),
        }
    }
}

async fn extract_jwt_claims<C: RoleChecker, S: CommonState>(
    req_parts: &mut Parts,
    state: &S,
) -> Outcome<Auth<C>, AuthException> {
    let ReqScopeModule(module) = ReqScopeModule::from_request_parts(req_parts, state)
        .await
        .context("failed to extract DiContainer")?;

    let Some(auth_header) = req_parts.headers.get(header::AUTHORIZATION) else {
        return Outcome::Exception(AuthException::MissingHeader);
    };

    let auth_header = auth_header
        .to_str()
        .context("failed to extract authorization header as str")?;

    if !auth_header.starts_with("Bearer ") {
        return Outcome::Exception(AuthException::UnsupportedScheme.into());
    }

    let bearer_token = &auth_header[7..];

    let claims = module
        .resolve::<TokenService>()
        .extract_claims(bearer_token)
        .await?;

    if !C::can_access(claims.role) {
        return Outcome::Exception(AuthException::NoRights.into());
    }

    Outcome::Success(Auth(claims, PhantomData))
}
