use shaku::Interface;

use crate::{
    errors::DomainError,
    models::{
        SecurityEntity,
        TradeEntity,
    },
};

#[cfg_attr(not(feature = "production"), mockall::automock)]
#[async_trait::async_trait]
pub trait ExchangeRepository: Interface + Send + Sync + 'static {
    async fn get_security(&self, security_id: &str) -> Result<SecurityEntity, DomainError>;
    async fn get_trades(&self, security_id: &str) -> Result<Vec<TradeEntity>, DomainError>;
}
