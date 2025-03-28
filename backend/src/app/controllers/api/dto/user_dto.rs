use crate::{
    app::controllers::api::rest_errors::ApiError,
    domain::{models::UserEntity, value_objects::Username},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetUserResponse {
    tg_id: i32,
    username: String,
    first_name: Option<String>,
    second_name: Option<String>,
}

impl From<UserEntity> for GetUserResponse {
    fn from(user_entity: UserEntity) -> Self {
        Self {
            tg_id: user_entity.tg_id,
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
            tg_id: response.tg_id,
            username,
            first_name: response.first_name,
            second_name: response.second_name,
        })
    }
}

pub type CreateUserResponse = GetUserResponse;
pub type CreateUserRequest = GetUserResponse;
