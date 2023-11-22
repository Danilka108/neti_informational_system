use async_trait::async_trait;
use axum::{extract::FromRequestParts, headers::UserAgent, RequestPartsExt, TypedHeader};
use http::request::Parts;

use crate::{api_state::ApiState, utils::CommonState};

pub struct SessionMetadata(pub String);

#[async_trait]
impl<S: CommonState> FromRequestParts<S> for SessionMetadata {
    type Rejection = <TypedHeader<UserAgent> as FromRequestParts<ApiState>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(user_agent) = parts.extract::<TypedHeader<UserAgent>>().await?;

        Ok(Self(user_agent.as_str().to_owned()))
    }
}
