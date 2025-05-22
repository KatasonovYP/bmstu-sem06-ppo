use crate::{
    app::api::api_errors::ApiError,
    domain::{
        models::ActiveEntity,
        value_objects::{
            Currency,
            Price,
        },
    },
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetActiveResponse {
    pub active_id: i32,
    pub user_id: i32,
    pub security_id: String,
    pub bought_price: f64,
    pub currency: String,
    pub count: i32,
}

impl From<ActiveEntity> for GetActiveResponse {
    fn from(active_entity: ActiveEntity) -> Self {
        Self {
            active_id: active_entity.active_id as i32,
            user_id: active_entity.user_id as i32,
            security_id: active_entity.security_id,
            bought_price: active_entity.bought_price.amount,
            currency: active_entity.bought_price.currency.value,
            count: active_entity.count as i32,
        }
    }
}

impl TryFrom<GetActiveResponse> for ActiveEntity {
    type Error = ApiError;

    fn try_from(active_response: GetActiveResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            active_id: active_response.active_id as u32,
            user_id: active_response.user_id as u32,
            security_id: active_response.security_id,
            bought_price: Price {
                amount: active_response.bought_price,
                currency: Currency {
                    value: active_response.currency,
                },
            },
            count: active_response.count as u32,
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateActiveRequest {
    pub user_id: i32,
    pub security_id: String,
    pub bought_price: f64,
    pub currency: String,
    pub count: i32,
}

impl From<ActiveEntity> for CreateActiveRequest {
    fn from(active_entity: ActiveEntity) -> Self {
        Self {
            user_id: active_entity.user_id as i32,
            security_id: active_entity.security_id,
            bought_price: active_entity.bought_price.amount,
            currency: active_entity.bought_price.currency.value,
            count: active_entity.count as i32,
        }
    }
}

impl TryFrom<CreateActiveRequest> for ActiveEntity {
    type Error = ApiError;

    fn try_from(active_response: CreateActiveRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: active_response.user_id as u32,
            security_id: active_response.security_id,
            bought_price: Price {
                amount: active_response.bought_price,
                currency: Currency {
                    value: active_response.currency,
                },
            },
            count: active_response.count as u32,
            ..Default::default()
        })
    }
}

pub type CreateActiveResponse = GetActiveResponse;
