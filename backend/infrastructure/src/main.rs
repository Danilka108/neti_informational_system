// #![feature(async_fn_in_trait)]
// #![feature(impl_trait_in_assoc_type)]

use std::sync::Arc;

use axum::Router;
use state::AppState;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod api;
mod auth;
mod env_config;
mod pg;
mod redis;
mod repositories;
mod state;

#[tokio::main]
async fn main() {
    dotenv::from_path("../../.env").unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(tracing_subscriber::fmt::time::SystemTime)
                .with_target(true)
                .with_file(true)
                .with_level(true)
                .pretty(),
        )
        .init();

    let env_config = match env_config::EnvConfig::try_load() {
        Ok(val) => val,
        Err(cause) => {
            tracing::error!(%cause, "failed to load environment config");
            return;
        }
    };

    let app_state = match AppState::new(env_config).await {
        Ok(val) => val,
        Err(cause) => {
            tracing::error!(%cause, "failed to init application state");
            return;
        }
    };

    let app = api::api().with_state(Arc::new(app_state));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
