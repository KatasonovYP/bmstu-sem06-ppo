use std::sync::Arc;

use axum::{
    Json,
    extract::{
        Path,
        State,
    },
};
use domain::ports::domain::AbstractUserService;
use telegram_authorizer::TelegramUser;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::{
    api_errors::ApiError,
    dto::user_dto::{
        UserRequest,
        UserResponse,
    },
};
#[derive(Clone)]
pub struct ApiUserController {
    pub user_service: Arc<dyn AbstractUserService>,
}

impl ApiUserController {
    pub fn new(user_service: Arc<dyn AbstractUserService>) -> Self {
        Self { user_service }
    }

    pub fn router(self) -> OpenApiRouter {
        OpenApiRouter::new()
            .routes(routes!(get_user, update_user, delete_user))
            .routes(routes!(list_users, create_user))
            .with_state(Arc::new(self))
    }
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "user",
    params(
        ("user_id", description = "User id"),
    ),
    responses(
        (status = 200, description = "Get user success", body = UserResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), ret)]
async fn get_user(
    State(controller): State<Arc<ApiUserController>>,
    Path(user_id): Path<u32>,
    TelegramUser {
        id,
        first_name,
        last_name,
        username,
    }: TelegramUser,
) -> Result<Json<UserResponse>, ApiError> {
    tracing::info!(id);
    controller
        .user_service
        .get_user(user_id)
        .await
        .map(UserResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    get,
    path = "",
    tag = "user",
    responses(
        (status = 200, description = "List user success", body = Vec<UserResponse>)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), ret)]
async fn list_users(
    State(controller): State<Arc<ApiUserController>>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users = controller.user_service.list_users().await?;

    let result = users
        .into_iter()
        .map(UserResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "",
    tag = "user",
    responses(
        (status = 200, description = "Create user success", body = UserResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), ret)]
async fn create_user(
    State(controller): State<Arc<ApiUserController>>,
    Json(user): Json<UserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let data = user.try_into()?;
    controller
        .user_service
        .create_user(data)
        .await
        .map(UserResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    patch,
    path = "/{user_id}",
    tag = "user",
    params(
        ("user_id", description = "User id"),
    ),
    responses(
        (status = 200, description = "Update user success", body = UserResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), ret)]
async fn update_user(
    State(controller): State<Arc<ApiUserController>>,
    Json(user): Json<UserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let data = user.try_into()?;
    controller
        .user_service
        .update_user(data)
        .await
        .map(UserResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    delete,
    path = "/{user_id}",
    tag = "user",
    params(
        ("user_id", description = "User id"),
    ),
    responses(
        (status = 200, description = "Delete user success", body = UserResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), ret)]
async fn delete_user(
    State(controller): State<Arc<ApiUserController>>,
    Path(user_id): Path<u32>,
) -> Result<Json<UserResponse>, ApiError> {
    controller
        .user_service
        .delete_user(user_id)
        .await
        .map(UserResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}
