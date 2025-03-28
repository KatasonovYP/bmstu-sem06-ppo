use crate::domain::{
    errors::DomainError,
    models::{ActiveEntity, NotificationEntity, UserEntity},
};
use async_trait::async_trait;

#[async_trait]
pub trait AbstractUserService: Send + Sync + 'static {
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError>;
    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, DomainError>;
}

#[async_trait]
pub trait AbstractActiveService: Send + Sync + 'static {
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError>;
    async fn create_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError>;
}

#[async_trait]
pub trait AbstractNotificationService: Send + Sync + 'static {
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError>;
    async fn create_notification(
        &self,
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError>;
}
