use std::sync::Arc;

use domain::{
    models::NotificationEntity,
    ports::domain::AbstractNotificationService,
};

use crate::commands::NotificationCommands;

pub struct CliNotificationController {
    notification_service: Arc<dyn AbstractNotificationService>,
}

impl CliNotificationController {
    pub fn new(notification_service: Arc<dyn AbstractNotificationService>) -> Self {
        Self {
            notification_service,
        }
    }

    pub async fn get_notification(&self, notification_id: &u32) {
        self.notification_service
            .get_notification(*notification_id)
            .await
            .unwrap();
    }

    pub async fn create_notification(&self, command: &NotificationCommands) {
        let entity = NotificationEntity::try_from(command).unwrap();
        self.notification_service
            .create_notification(entity)
            .await
            .unwrap();
    }

    pub async fn update_notification(&self, command: &NotificationCommands) {
        let entity = NotificationEntity::try_from(command).unwrap();
        self.notification_service
            .update_notification(entity)
            .await
            .unwrap();
    }

    pub async fn list_notifications(&self) {
        self.notification_service
            .list_notifications()
            .await
            .unwrap();
    }

    pub async fn list_active_notifications(&self, active_id: &u32) {
        self.notification_service
            .list_active_notifications(*active_id)
            .await
            .unwrap();
    }

    pub async fn delete_notification(&self, notification_id: &u32) {
        self.notification_service
            .delete_notification(*notification_id)
            .await
            .unwrap();
    }
}
