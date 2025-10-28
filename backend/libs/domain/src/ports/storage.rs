use shaku::Interface;

use crate::{
    errors::DomainError,
    models::{
        ActiveEntity,
        NotificationEntity,
        SentEntity,
        UserEntity,
    },
};

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepository: Interface + Send + Sync + 'static {
    async fn create_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError>;
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError>;
    async fn get_user_by_tg_id(&self, tg_id: i64) -> Result<UserEntity, DomainError>;
    async fn list_users(&self) -> Result<Vec<UserEntity>, DomainError>;
    async fn update_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError>;
    async fn delete_user(&self, user_id: u32) -> Result<UserEntity, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait ActiveRepository: Interface + Send + Sync + 'static {
    async fn create_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError>;
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError>;
    async fn list_actives(&self) -> Result<Vec<ActiveEntity>, DomainError>;
    async fn list_user_actives(&self, user_id: u32) -> Result<Vec<ActiveEntity>, DomainError>;
    async fn update_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError>;
    async fn delete_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait NotificationRepository: Interface + Send + Sync + 'static {
    async fn create_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError>;
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError>;
    async fn list_notifications(&self) -> Result<Vec<NotificationEntity>, DomainError>;
    async fn list_active_notifications(
        &self,
        active_id: u32,
    ) -> Result<Vec<NotificationEntity>, DomainError>;
    async fn update_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError>;
    async fn delete_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait SentRepository: Interface + Send + Sync + 'static {
    async fn create_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError>;
    async fn get_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError>;
    async fn list_sent(&self) -> Result<Vec<SentEntity>, DomainError>;
    async fn update_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError>;
    async fn delete_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError>;
    async fn create_sent_now(&self, notification_id: u32) -> Result<SentEntity, DomainError>;
}
