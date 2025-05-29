use shaku::Interface;

use crate::{
    errors::DomainError,
    value_objects::Price,
};

#[cfg_attr(not(feature = "production"), mockall::automock)]
#[async_trait::async_trait]
pub trait PriceCacheRepository: Interface + Send + Sync + 'static {
    async fn set_price(&self, security_id: &str, price: Price) -> Result<Price, DomainError>;
    async fn get_price(&self, security_id: &str) -> Result<Price, DomainError>;
}
