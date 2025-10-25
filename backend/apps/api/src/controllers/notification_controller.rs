use std::sync::Arc;

use axum::{
    Json,
    extract::{
        Path,
        State,
    },
};
use domain::ports::domain::AbstractNotificationService;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::{
    api_errors::ApiError,
    dto::notification_dto::{
        NotificationRequest,
        NotificationResponse,
    },
};
#[derive(Clone)]
pub struct ApiNotificationController {
    pub notification_service: Arc<dyn AbstractNotificationService>,
}

impl ApiNotificationController {
    pub fn new(notification_service: Arc<dyn AbstractNotificationService>) -> Self {
        Self {
            notification_service,
        }
    }

    pub fn router(self) -> OpenApiRouter {
        OpenApiRouter::new()
            .routes(routes!(
                get_notification,
                update_notification,
                delete_notification
            ))
            .routes(routes!(list_active_notifications, create_notification))
            .with_state(Arc::new(self))
    }
}

#[utoipa::path(
    get,
    path = "/{notification_id}",
    tag = "notification",
    params(
        ("notification_id", description = "Notification id"),
    ),
    responses(
        (status = 200, description = "Get notification success", body = NotificationResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn get_notification(
    State(controller): State<Arc<ApiNotificationController>>,
    Path(notification_id): Path<u32>,
) -> Result<Json<NotificationResponse>, ApiError> {
    controller
        .notification_service
        .get_notification(notification_id)
        .await
        .map(NotificationResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    get,
    path = "/all/{active_id}",
    tag = "notification",
    params(
        ("active_id", description = "Active id"),
    ),
    responses(
        (status = 200, description = "List notification success", body = Vec<NotificationResponse>)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn list_active_notifications(
    State(controller): State<Arc<ApiNotificationController>>,
    Path(active_id): Path<u32>,
) -> Result<Json<Vec<NotificationResponse>>, ApiError> {
    let notifications = controller
        .notification_service
        .list_active_notifications(active_id)
        .await?;

    let result = notifications
        .into_iter()
        .map(NotificationResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "",
    tag = "notification",
    responses(
        (status = 200, description = "Create notification success", body = NotificationResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn create_notification(
    State(controller): State<Arc<ApiNotificationController>>,
    Json(notification): Json<NotificationRequest>,
) -> Result<Json<NotificationResponse>, ApiError> {
    let data = notification.try_into()?;
    controller
        .notification_service
        .create_notification(data)
        .await
        .map(NotificationResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    patch,
    path = "/{notification_id}",
    tag = "notification",
    params(
        ("notification_id" = u32, Path, description = "Notification id"),
    ),
    responses(
        (status = 200, description = "Update notification success", body = NotificationResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn update_notification(
    State(controller): State<Arc<ApiNotificationController>>,
    Json(notification): Json<NotificationRequest>,
) -> Result<Json<NotificationResponse>, ApiError> {
    let data = notification.try_into()?;
    controller
        .notification_service
        .update_notification(data)
        .await
        .map(NotificationResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    delete,
    path = "/{notification_id}",
    tag = "notification",
    params(
        ("notification_id", description = "Notification id"),
    ),
    responses(
        (status = 200, description = "Delete notification success", body = NotificationResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn delete_notification(
    State(controller): State<Arc<ApiNotificationController>>,
    Path(notification_id): Path<u32>,
) -> Result<Json<NotificationResponse>, ApiError> {
    controller
        .notification_service
        .delete_notification(notification_id)
        .await
        .map(NotificationResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}
