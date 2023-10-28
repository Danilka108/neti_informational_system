use std::sync::Arc;

use crate::adapters::AdaptersModule;
use crate::config::env_config::EnvConfig;
use crate::pg::PgTransaction;
use app::AppModule;
use tokio::sync::Mutex;

use crate::config::ConfigContainer;

use crate::pg::init_pg_conn_pool;

#[derive(Debug, Clone)]
pub struct AppState {
    config_container: ConfigContainer,
    pg_pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateError {
    #[error("failed to init postgres connection pool")]
    InitPgConnPoolError(#[from] sqlx::Error),
}

impl AppState {
    pub async fn try_from_config(env_config: EnvConfig) -> Result<Self, AppStateError> {
        let pg_pool = init_pg_conn_pool(&env_config).await?;
        let config_container = ConfigContainer::from(&env_config);

        Ok(Self {
            config_container,
            pg_pool,
        })
    }

    pub async fn begin(
        &self,
    ) -> Result<
        (
            AppModule<AdaptersModule<ConfigContainer>>,
            Arc<Mutex<PgTransaction>>,
        ),
        sqlx::Error,
    > {
        let txn = Arc::new(Mutex::new(self.pg_pool.begin().await?));

        let adapters_container = AdaptersModule {
            config: self.config_container.clone(),
            txn: Arc::clone(&txn),
        };

        let app_container = AppModule {
            adapters: adapters_container,
        };

        Ok((app_container, txn))
    }
}
