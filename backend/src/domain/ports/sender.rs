use async_trait::async_trait;
use shaku::Interface;

use crate::domain::errors::DomainError;

#[cfg_attr(not(feature = "production"), mockall::automock)]
#[async_trait]
pub trait NotificationSender: Interface + Send + Sync + 'static {
    async fn send_message(&self, chat_id: i64, message: String) -> Result<String, DomainError>;
}
