use async_trait::async_trait;
use axum::{
    extract::{FromRequest, Request},
    Json,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::AppState;

use super::api_error::ApiError;

pub struct JsonWithRejection<J>(pub J);

#[async_trait]
impl<J: for<'de> Deserialize<'de>> FromRequest<AppState> for JsonWithRejection<J> {
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let WithRejection(Json(value), _) =
            WithRejection::<Json<J>, ApiError>::from_request(req, state).await?;

        Ok(JsonWithRejection(value))
    }
}
