use std::sync::Arc;

use aide::{
    axum::IntoApiResponse,
    transform::TransformOperation,
};
use axum::{
    extract::{
        Path,
        State,
    },
    response::IntoResponse,
    Json,
};
use http::StatusCode;

use crate::{
    app::api::{
        api_errors::ApiError,
        dto::user_dto::{
            CreateUserRequest,
            CreateUserResponse,
            GetUserResponse,
        },
    },
    domain::ports::domain::AbstractUserService,
};

#[derive(Clone)]
pub struct ApiUserController {
    user_service: Arc<dyn AbstractUserService>,
}

impl ApiUserController {
    pub fn new(user_service: Arc<dyn AbstractUserService>) -> Self {
        Self { user_service }
    }

    // #[tracing::instrument(skip(controller), ret)]
    pub async fn get_user(
        State(_controller): State<Self>,
        Path(_user_id): Path<u32>,
    ) -> (StatusCode, Result<Json<GetUserResponse>, ApiError>) {
        todo!()
        // let result = controller
        //     .user_service
        //     .get_user(user_id)
        //     .await
        //     .map(GetUserResponse::from)
        //     .map(Json)
        //     .map_err(ApiError::from);
        // match result {
        //     Ok(entity) => (StatusCode::OK, Ok(entity)),
        //     Err(err) => (err.clone().into_response().status(), Err(err)),
        // }
    }

    pub fn get_user_docs(op: TransformOperation) -> TransformOperation {
        op.description("Get a user")
            .response::<200, Json<GetUserResponse>>()
    }

    #[tracing::instrument(skip(controller), ret)]
    pub async fn create_user(
        State(controller): State<Self>,
        Json(new_user): Json<CreateUserRequest>,
    ) -> impl IntoApiResponse {
        let user_data = match new_user.try_into() {
            Ok(data) => data,
            Err(err) => return (StatusCode::BAD_REQUEST, Err(err)),
        };

        match controller.user_service.create_user(user_data).await {
            Ok(user) => (
                StatusCode::CREATED,
                Ok(Json(CreateUserResponse::from(user))),
            ),
            Err(err) => (
                ApiError::from(err.clone()).into_response().status(),
                Err(ApiError::from(err)),
            ),
        }
    }

    pub fn create_user_docs(op: TransformOperation) -> TransformOperation {
        op.description("Create a new user")
            .response::<201, Json<CreateUserResponse>>()
    }

    #[tracing::instrument(skip(controller), ret)]
    pub async fn update_user(
        State(controller): State<Self>,
        Json(new_user): Json<CreateUserRequest>,
    ) -> impl IntoApiResponse {
        let user_data = match new_user.try_into() {
            Ok(data) => data,
            Err(err) => return (StatusCode::BAD_REQUEST, Err(err)),
        };

        match controller.user_service.create_user(user_data).await {
            Ok(user) => (
                StatusCode::CREATED,
                Ok(Json(CreateUserResponse::from(user))),
            ),
            Err(err) => (
                ApiError::from(err.clone()).into_response().status(),
                Err(ApiError::from(err)),
            ),
        }
    }

    pub fn update_user_docs(op: TransformOperation) -> TransformOperation {
        op.description("Update a user")
            .response::<201, Json<CreateUserResponse>>()
    }
}
