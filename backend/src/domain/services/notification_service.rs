use crate::domain::errors::DomainError;
use crate::domain::models::NotificationEntity;
use crate::ports::inbound::domain::AbstractNotificationService;
use crate::ports::outbound::db::NotificationRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct NotificationService {
    notification_repository: Arc<dyn NotificationRepository>,
}

impl NotificationService {
    pub fn new(notification_repository: Arc<dyn NotificationRepository>) -> Self {
        Self {
            notification_repository,
        }
    }
}

#[async_trait]
impl AbstractNotificationService for NotificationService {
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        self.notification_repository
            .get_notification(notification_id)
            .await
    }

    async fn create_notification(
        &self,
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        self.notification_repository
            .create_notification(notification)
            .await
    }
}
