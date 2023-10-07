mod auth;
mod common;
mod user;

use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub fn api() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
}
