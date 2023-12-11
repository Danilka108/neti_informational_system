#![feature(iterator_try_collect)]
#![feature(try_trait_v2)]

use config::ConfigModule;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod api_state;
mod config;
mod handlers;
mod utils;

fn test() {
    let a = app::subdivision::Entity {
        id: Default::default(),
        name: "".into(),
        university_id: Default::default(),
        tags: utils::entity::LazyAttr,
        members: utils::entity::LazyAttr,
    };
    let t: adapters::transaction::TransactionModule<ConfigModule> = todo!();
    let a = a.tags(&0.into_id()).exec(t).await;
    match a {
        ::utils::outcome::Outcome::Ok(_) => (),
        _ => (),
    }
}

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

    let api_state = match api_state::ApiState::new(env_config.into()).await {
        Ok(val) => val,
        Err(cause) => {
            tracing::error!(%cause, "failed to boot server");
            return;
        }
    };

    let api = handlers::router(api_state);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(api.into_make_service())
        .await
        .unwrap();
}
