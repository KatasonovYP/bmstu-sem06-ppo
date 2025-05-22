use std::sync::Arc;

use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};

use crate::{
    app::api::{
        api_errors::ApiError,
        dto::notification_dto::{
            CreateNotificationRequest,
            CreateNotificationResponse,
            GetNotificationResponse,
        },
    },
    domain::ports::domain::AbstractNotificationService,
};

#[derive(Clone)]
pub struct ApiNotificationController {
    notification_service: Arc<dyn AbstractNotificationService>,
}

impl ApiNotificationController {
    pub fn new(notification_service: Arc<dyn AbstractNotificationService>) -> Self {
        Self {
            notification_service,
        }
    }

    #[tracing::instrument(skip(controller), err(Debug), ret)]
    pub async fn get_notification(
        State(controller): State<Self>,
        Path(notification_id): Path<u32>,
    ) -> Result<Json<GetNotificationResponse>, ApiError> {
        controller
            .notification_service
            .get_notification(notification_id)
            .await
            .map(GetNotificationResponse::from)
            .map(Json)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(controller), err(Debug), ret)]
    pub async fn create_notification(
        State(controller): State<Self>,
        Json(new_notification): Json<CreateNotificationRequest>,
    ) -> Result<Json<CreateNotificationResponse>, ApiError> {
        controller
            .notification_service
            .create_notification(new_notification.into())
            .await
            .map(CreateNotificationResponse::from)
            .map(Json)
            .map_err(ApiError::from)
    }
    // pub async fn notify(
    //     State(controller): State<Self>,
    //     Path(chat_id): Path<i64>,
    // ) -> Json<()> {
    //     controller.notification_service.notify(chat_id, "hello").await.unwrap();
    //     Json(())
    // }
}
