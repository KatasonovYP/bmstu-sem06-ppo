use std::sync::Arc;

use super::dto::notification_dto::{
    CreateNotificationRequest, CreateNotificationResponse, GetNotificationResponse,
};
use crate::ports::inbound::domain::AbstractNotificationService;
use axum::{
    extract::{Path, State},
    Json,
};

#[derive(Clone)]
pub struct NotificationController {
    notification_service: Arc<dyn AbstractNotificationService>,
}

impl NotificationController {
    pub fn new(notification_service: Arc<dyn AbstractNotificationService>) -> Self {
        Self {
            notification_service,
        }
    }

    pub async fn get_notification(
        State(controller): State<NotificationController>,
        Path(notification_id): Path<u32>,
    ) -> Json<GetNotificationResponse> {
        controller
            .notification_service
            .get_notification(notification_id)
            .await
            .map(GetNotificationResponse::from)
            .map(Json)
            .unwrap()
    }

    pub async fn create_notification(
        State(controller): State<NotificationController>,
        Json(new_notification): Json<CreateNotificationRequest>,
    ) -> Json<CreateNotificationResponse> {
        controller
            .notification_service
            .create_notification(new_notification.into())
            .await
            .map(CreateNotificationResponse::from)
            .map(Json)
            .unwrap()
    }
}
