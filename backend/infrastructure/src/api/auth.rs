use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
struct LoginPayload {
    email: Box<str>,
    password: Box<str>,
}

async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let a = sqlx::query!("SELECT * FROM users WHERE email = $1", "test@test.test")
        .fetch_one(app_state.pg_pool())
        .await
        .unwrap();

    dbg!(a);

    "ok"
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/login", post(login))
}
