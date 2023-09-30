use self::{check_role::RoleChecker, error::AuthError};
use crate::{env_config::EnvConfig, state::AppState};

use axum::extract::FromRequestParts;
use http::header;
use http::request::Parts;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, TokenData, Validation};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

mod check_role;
mod error;

pub mod roles;

pub fn gen_jwt_keys(env_config: &EnvConfig) -> (EncodingKey, DecodingKey) {
    (
        EncodingKey::from_secret(env_config.jwt_secret.as_bytes()),
        DecodingKey::from_secret(&env_config.jwt_secret.as_bytes()),
    )
}

#[derive(Deserialize)]
pub struct Claims {
    pub user_id: u32,
    pub issued_at: u64,
    pub role: String,
}

pub struct Auth<C: RoleChecker> {
    pub claims: Claims,
    _checker: std::marker::PhantomData<C>,
}

#[async_trait::async_trait]
impl<C: RoleChecker> FromRequestParts<AppState> for Auth<C> {
    type Rejection = AuthError;

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

        try_authenticate(auth_header, app_state).await
    }
}

async fn try_authenticate<C: RoleChecker>(
    auth_header: &str,
    app_state: &AppState,
) -> Result<Auth<C>, AuthError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::UnsupportedScheme);
    }

    let bearer_token = &auth_header[7..];
    let decoding_key = app_state.jwt_decoding_key();

    let Ok(token_data) = jsonwebtoken::decode::<Claims>(
        &bearer_token,
        decoding_key,
        &Validation::new(Algorithm::HS256),
    ) else {
        return Err(AuthError::InvalidToken);
    };

    validate_token_expiration(&token_data)?;

    if !C::can_access(&token_data.claims.role) {
        return Err(AuthError::Forbidden);
    }

    Ok(Auth {
        claims: token_data.claims,
        _checker: std::marker::PhantomData,
    })
}

fn validate_token_expiration(token_data: &TokenData<Claims>) -> Result<(), AuthError> {
    let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return Err(AuthError::InternalError);
    };

    if now.as_secs() > token_data.claims.issued_at {
        return Err(AuthError::ExpiredToken);
    }

    Ok(())
}
