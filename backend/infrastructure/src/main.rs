#![feature(iterator_try_collect)]
#![feature(try_trait_v2)]

use state::AppState;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod adapters;
mod config;
mod handlers;
mod pg;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::from_path(".env").unwrap();

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

    let env_config = match config::env_config::EnvConfig::try_load() {
        Ok(val) => val,
        Err(cause) => {
            tracing::error!(%cause, "failed to boot server");
            return;
        }
    };

    let app_state = match AppState::try_from_config(env_config).await {
        Ok(val) => val,
        Err(cause) => {
            tracing::error!(%cause, "failed to boot server");
            return;
        }
    };

    let app = handlers::router(app_state);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
