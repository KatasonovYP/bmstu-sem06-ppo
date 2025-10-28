use shaku::Interface;

use crate::{
    errors::DomainError,
    models::{
        ActiveEntity,
        NotificationEntity,
        SentEntity,
        UserEntity,
    },
    value_objects::Price,
};

#[mockall::automock]
#[async_trait::async_trait]
pub trait AbstractUserService: Interface + Send + Sync + 'static {
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError>;
    async fn get_user_by_tg_id(&self, tg_id: i64) -> Result<UserEntity, DomainError>;
    async fn create_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError>;
    async fn list_users(&self) -> Result<Vec<UserEntity>, DomainError>;
    async fn update_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError>;
    async fn delete_user(&self, user_id: u32) -> Result<UserEntity, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait AbstractActiveService: Interface + Send + Sync + 'static {
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError>;
    async fn create_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError>;
    async fn list_actives(&self) -> Result<Vec<ActiveEntity>, DomainError>;
    async fn list_user_actives(&self, user_id: u32) -> Result<Vec<ActiveEntity>, DomainError>;
    async fn update_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError>;
    async fn delete_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError>;
    async fn sum_user_actives_bought_price(&self, user_id: u32) -> Result<Price, DomainError>;
    async fn sum_user_actives_current_price(&self, user_id: u32) -> Result<Price, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait AbstractNotificationService: Interface + Send + Sync + 'static {
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError>;
    async fn list_notifications(&self) -> Result<Vec<NotificationEntity>, DomainError>;
    async fn list_active_notifications(
        &self,
        active_id: u32,
    ) -> Result<Vec<NotificationEntity>, DomainError>;
    async fn create_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError>;
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
pub trait AbstractSentService: Interface + Send + Sync + 'static {
    async fn get_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError>;
    async fn create_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError>;
    async fn list_sent(&self) -> Result<Vec<SentEntity>, DomainError>;
    async fn update_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError>;
    async fn delete_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait AbstractPriceOpsService: Interface + Send + Sync + 'static {
    async fn get_actives_bought_price(
        &self,
        actives: Vec<ActiveEntity>,
    ) -> Result<Price, DomainError>;
    async fn get_actives_current_price(
        &self,
        actives: Vec<ActiveEntity>,
    ) -> Result<Price, DomainError>;
    async fn get_active_current_price(&self, active: &ActiveEntity) -> Result<Price, DomainError>;
    async fn get_security_current_price(&self, security_id: &str) -> Result<Price, DomainError>;
    async fn get_price_delta(&self, active: &ActiveEntity) -> Result<Price, DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait AbstractLimitMonitorService: Interface + Send + Sync + 'static {
    async fn send_exeeding_messages(&self) -> Result<(), DomainError>;
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait AbstractPriceCacheService: Interface + Send + Sync + 'static {
    async fn get_price(&self, active: &ActiveEntity) -> Result<Price, DomainError>;
    async fn refresh_security_price(&self, security_id: &str) -> Result<Price, DomainError>;
    async fn refresh_all_prices(&self) -> Result<(), DomainError>;
}
