use axum::Router;

use crate::AppState;

mod university;

pub fn handlers() -> Router<AppState> {
    Router::new()
}
