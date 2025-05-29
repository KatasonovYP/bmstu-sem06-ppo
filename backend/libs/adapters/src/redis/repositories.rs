use std::sync::Arc;

use domain::{
    errors::DomainError,
    ports::cache::PriceCacheRepository,
    value_objects::Price,
};
use redis::AsyncCommands;
use shaku::Component;
use tokio::sync::Mutex;

#[derive(Component)]
#[shaku(interface = PriceCacheRepository)]
pub struct RedisPriceCacheRepository {
    connection: Arc<Mutex<redis::aio::MultiplexedConnection>>,
}

#[async_trait::async_trait]
impl PriceCacheRepository for RedisPriceCacheRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn set_price(&self, security_id: &str, price: Price) -> Result<Price, DomainError> {
        let mut conn = self.connection.lock().await;
        conn.set::<_, _, ()>(security_id, price.clone().amount)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        Ok(price)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_price(&self, security_id: &str) -> Result<Price, DomainError> {
        let mut conn = self.connection.lock().await;
        let amount = conn
            .get(security_id)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        Ok(Price::rub(amount))
    }
}
