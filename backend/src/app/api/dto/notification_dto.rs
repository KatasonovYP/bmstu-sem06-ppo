use crate::domain::{
    models::NotificationEntity,
    value_objects::Price,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetNotificationResponse {
    pub notification_id: i32,
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: f64,
    pub limit_lower: f64,
    pub limit_type: String,
}

impl From<NotificationEntity> for GetNotificationResponse {
    fn from(notification_entity: NotificationEntity) -> Self {
        Self {
            notification_id: notification_entity.notification_id as i32,
            portfolio_id: notification_entity.portfolio_id as i32,
            active_id: notification_entity.active_id as i32,
            limit_upper: notification_entity.limit_upper.amount,
            limit_lower: notification_entity.limit_lower.amount,
            limit_type: notification_entity.limit_upper.currency.value,
        }
    }
}

impl From<GetNotificationResponse> for NotificationEntity {
    fn from(notification_response: GetNotificationResponse) -> Self {
        let limit_upper = Price::new(
            notification_response.limit_upper,
            notification_response.limit_type.clone(),
        );
        let limit_lower = Price::new(
            notification_response.limit_lower,
            notification_response.limit_type.clone(),
        );
        Self {
            portfolio_id: notification_response.portfolio_id as u32,
            active_id: notification_response.active_id as u32,
            limit_upper,
            limit_lower,
            ..Default::default()
        }
    }
}

pub type CreateNotificationResponse = GetNotificationResponse;
pub type CreateNotificationRequest = GetNotificationResponse;
