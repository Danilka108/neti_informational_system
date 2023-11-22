use adapters::transaction::TransactionModule;

#[derive(Debug, Clone)]
pub struct ApiState {
    adapters_module: adapters::AdaptersModule<crate::config::ConfigModule>,
}

impl ApiState {
    pub async fn new(config: crate::config::ConfigModule) -> Result<Self, anyhow::Error> {
        Ok(Self {
            adapters_module: adapters::AdaptersModule::new(config).await?,
        })
    }

    pub async fn begin_request_scope(
        &self,
    ) -> Result<TransactionModule<crate::config::ConfigModule>, anyhow::Error> {
        self.adapters_module.begin_transaction_scope().await
    }
}
