use crate::domain::models::ActiveEntity;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetActiveResponse {
    pub user_id: i32,
    pub security_id: i32,
    pub bought_price: i32,
    pub count: i32,
}

impl From<ActiveEntity> for GetActiveResponse {
    fn from(active_entity: ActiveEntity) -> Self {
        Self {
            user_id: active_entity.user_id,
            security_id: active_entity.security_id,
            bought_price: active_entity.bought_price,
            count: active_entity.count,
        }
    }
}

impl From<GetActiveResponse> for ActiveEntity {
    fn from(active_response: GetActiveResponse) -> Self {
        Self {
            user_id: active_response.user_id,
            security_id: active_response.security_id,
            bought_price: active_response.bought_price,
            count: active_response.count,
        }
    }
}

pub type CreateActiveResponse = GetActiveResponse;
pub type CreateActiveRequest = GetActiveResponse;
