use domain::{
    models::NotificationEntity,
    value_objects::Price,
};

use crate::api_errors::ApiError;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct NotificationResponse {
    pub notification_id: i32,
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: f64,
    pub limit_lower: f64,
    pub limit_type: String,
    pub resend_interval_sec: i32,
}

impl From<NotificationEntity> for NotificationResponse {
    fn from(notification_entity: NotificationEntity) -> Self {
        Self {
            notification_id: notification_entity.notification_id as i32,
            portfolio_id: notification_entity.portfolio_id as i32,
            active_id: notification_entity.active_id as i32,
            limit_upper: notification_entity.limit_upper.amount,
            limit_lower: notification_entity.limit_lower.amount,
            limit_type: notification_entity.limit_upper.currency.value,
            resend_interval_sec: notification_entity.resend_interval_sec as i32,
        }
    }
}

impl From<NotificationResponse> for NotificationEntity {
    fn from(notification_response: NotificationResponse) -> Self {
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
            resend_interval_sec: notification_response.resend_interval_sec as u32,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct NotificationRequest {
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: f64,
    pub limit_lower: f64,
    pub limit_type: String,
    pub resend_interval_sec: i32,
}

impl From<NotificationEntity> for NotificationRequest {
    fn from(notification_entity: NotificationEntity) -> Self {
        Self {
            portfolio_id: notification_entity.portfolio_id as i32,
            active_id: notification_entity.active_id as i32,
            limit_upper: notification_entity.limit_upper.amount,
            limit_lower: notification_entity.limit_lower.amount,
            limit_type: notification_entity.limit_upper.currency.value,
            resend_interval_sec: notification_entity.resend_interval_sec as i32,
        }
    }
}

impl TryFrom<NotificationRequest> for NotificationEntity {
    type Error = ApiError;

    fn try_from(notification_request: NotificationRequest) -> Result<Self, Self::Error> {
        let limit_upper = Price::new(
            notification_request.limit_upper,
            notification_request.limit_type.clone(),
        );
        let limit_lower = Price::new(
            notification_request.limit_lower,
            notification_request.limit_type.clone(),
        );

        Ok(Self {
            portfolio_id: notification_request.portfolio_id as u32,
            active_id: notification_request.active_id as u32,
            limit_upper,
            limit_lower,
            resend_interval_sec: notification_request.resend_interval_sec as u32,
            ..Default::default()
        })
    }
}
