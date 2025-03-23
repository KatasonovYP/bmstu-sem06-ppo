use crate::domain::models::NotificationEntity;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetNotificationResponse {
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: i32,
    pub limit_lower: i32,
    pub limit_type: i16,
}

impl From<NotificationEntity> for GetNotificationResponse {
    fn from(notification_entity: NotificationEntity) -> Self {
        Self {
            portfolio_id: notification_entity.portfolio_id,
            active_id: notification_entity.active_id,
            limit_upper: notification_entity.limit_upper,
            limit_lower: notification_entity.limit_lower,
            limit_type: notification_entity.limit_type,
        }
    }
}

impl From<GetNotificationResponse> for NotificationEntity {
    fn from(notification_response: GetNotificationResponse) -> Self {
        Self {
            portfolio_id: notification_response.portfolio_id,
            active_id: notification_response.active_id,
            limit_upper: notification_response.limit_upper,
            limit_lower: notification_response.limit_lower,
            limit_type: notification_response.limit_type,
        }
    }
}

pub type CreateNotificationResponse = GetNotificationResponse;
pub type CreateNotificationRequest = GetNotificationResponse;
