use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::ports::inbound::domain::AbstractUserService;

use super::{
    dto::user_dto::{CreateUserRequest, CreateUserResponse, GetUserResponse},
    rest_errors::ApiError,
};

#[derive(Clone)]
pub struct UserController {
    user_service: Arc<dyn AbstractUserService>,
}

impl UserController {
    pub fn new(user_service: Arc<dyn AbstractUserService>) -> Self {
        Self { user_service }
    }

    pub async fn get_user(
        State(controller): State<UserController>,
        Path(user_id): Path<u32>,
    ) -> Result<Json<CreateUserResponse>, ApiError> {
        let user = controller
            .user_service
            .get_user(user_id)
            .await
            .map(GetUserResponse::from)
            .map(Json)
            .map_err(ApiError::from)?;

        Ok(user)
    }

    pub async fn create_user(
        State(controller): State<UserController>,
        Json(new_user): Json<CreateUserRequest>,
    ) -> Result<Json<CreateUserResponse>, ApiError> {
        let user_data = new_user.try_into().map_err(ApiError::from)?;

        let user = controller
            .user_service
            .create_user(user_data)
            .await
            .map_err(ApiError::from)?;

        Ok(Json(CreateUserResponse::from(user)))
    }
}
