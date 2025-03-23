use axum::{
    extract::{Path, State},
    Json,
};

use crate::domain::services::user_service::UserService;

use super::dto::user_dto::{CreateUserRequest, CreateUserResponse, GetUserResponse};

#[derive(Clone)]
pub struct UserController {
    user_service: UserService,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    pub async fn get_user(
        State(controller): State<UserController>,
        Path(user_id): Path<u32>,
    ) -> Json<GetUserResponse> {
        controller
            .user_service
            .get_user(user_id)
            .await
            .map(GetUserResponse::from)
            .map(Json)
            .unwrap()
    }

    pub async fn create_user(
        State(controller): State<UserController>,
        Json(new_user): Json<CreateUserRequest>,
    ) -> Json<GetUserResponse> {
        controller
            .user_service
            .create_user(new_user.into())
            .await
            .map(CreateUserResponse::from)
            .map(Json)
            .unwrap()
    }
}
