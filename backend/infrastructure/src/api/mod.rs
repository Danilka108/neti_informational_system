use crate::AppState;
use axum::Router;
use std::sync::Arc;

mod auth;

pub fn api() -> Router<Arc<AppState>> {
    Router::new().nest("/auth", auth::router())
}
