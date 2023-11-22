mod auth;
mod user;

use crate::{api_state::ApiState, utils::provide_req_scope_module};
use axum::{middleware, Router};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            provide_req_scope_module,
        ))
        .with_state(state)
}
