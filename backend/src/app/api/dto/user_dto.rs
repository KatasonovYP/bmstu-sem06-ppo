use schemars::JsonSchema;

use crate::{
    app::api::api_errors::ApiError,
    domain::{
        models::UserEntity,
        value_objects::Username,
    },
};

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, JsonSchema)]
pub struct GetUserResponse {
    user_id: i32,
    tg_id: i64,
    chat_id: i64,
    username: String,
    first_name: Option<String>,
    second_name: Option<String>,
}

impl From<UserEntity> for GetUserResponse {
    fn from(user_entity: UserEntity) -> Self {
        Self {
            user_id: user_entity.user_id as i32,
            tg_id: user_entity.tg_id,
            chat_id: user_entity.chat_id,
            username: user_entity.username.value,
            first_name: user_entity.first_name,
            second_name: user_entity.second_name,
        }
    }
}

impl TryFrom<GetUserResponse> for UserEntity {
    type Error = ApiError;

    fn try_from(response: GetUserResponse) -> Result<Self, Self::Error> {
        let username = Username::new(response.username).map_err(ApiError::from)?;

        Ok(Self {
            user_id: response.user_id as u32,
            tg_id: response.tg_id,
            chat_id: response.chat_id,
            username,
            first_name: response.first_name,
            second_name: response.second_name,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, JsonSchema)]
pub struct CreateUserRequest {
    tg_id: i64,
    chat_id: i64,
    username: String,
    first_name: Option<String>,
    second_name: Option<String>,
}

impl From<UserEntity> for CreateUserRequest {
    fn from(user_entity: UserEntity) -> Self {
        Self {
            tg_id: user_entity.tg_id,
            chat_id: user_entity.chat_id,
            username: user_entity.username.value,
            first_name: user_entity.first_name,
            second_name: user_entity.second_name,
        }
    }
}

impl TryFrom<CreateUserRequest> for UserEntity {
    type Error = ApiError;

    fn try_from(response: CreateUserRequest) -> Result<Self, Self::Error> {
        let username = Username::new(response.username).map_err(ApiError::from)?;

        Ok(Self {
            tg_id: response.tg_id,
            chat_id: response.chat_id,
            username,
            first_name: response.first_name,
            second_name: response.second_name,
            ..Default::default()
        })
    }
}

pub type CreateUserResponse = GetUserResponse;
