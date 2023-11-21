mod auth;
mod user;

use crate::{utils::provide_di_container, AppState};
use axum::{middleware, Router};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            provide_di_container,
        ))
        .with_state(state)
}
