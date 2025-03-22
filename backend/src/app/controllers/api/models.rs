use crate::domain::models::UserEntity;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetUserResponse {
    tg_id: i32,
    username: String,
    first_name: String,
    second_name: String,
}

impl From<UserEntity> for GetUserResponse {
    fn from(user_entity: UserEntity) -> Self {
        Self {
            tg_id: user_entity.tg_id,
            username: user_entity.username,
            first_name: user_entity.first_name,
            second_name: user_entity.second_name,
        }
    }
}

impl From<GetUserResponse> for UserEntity {
    fn from(val: GetUserResponse) -> Self {
        Self {
            tg_id: val.tg_id,
            username: val.username,
            first_name: val.first_name,
            second_name: val.second_name,
        }
    }
}

pub type CreateUserResponse = GetUserResponse;
pub type CreateUserRequest = GetUserResponse;
