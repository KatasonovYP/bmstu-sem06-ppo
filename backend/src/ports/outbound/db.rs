use crate::domain::errors::DomainError;
use crate::domain::models::{ActiveEntity, NotificationEntity, UserEntity};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_user(&self, tg_id: u32) -> Result<UserEntity, DomainError>;
    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, DomainError>;
}

#[async_trait]
pub trait ActiveRepository: Send + Sync + 'static {
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError>;
    async fn create_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError>;
}

#[async_trait]
pub trait NotificationRepository: Send + Sync + 'static {
    async fn get_notification(&self, id: u32) -> Result<NotificationEntity, DomainError>;
    async fn create_notification(
        &self,
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError>;
}
