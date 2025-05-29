use std::sync::Arc;

use axum::{
    Extension,
    Json,
    extract::{
        Path,
        State,
    },
};
use domain::ports::domain::AbstractActiveService;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::{
    api_errors::ApiError,
    dto::active_dto::{
        ActiveRequest,
        ActiveResponse,
    },
};
#[derive(Clone)]
pub struct ApiActiveController {
    pub active_service: Arc<dyn AbstractActiveService>,
}

impl ApiActiveController {
    pub fn new(active_service: Arc<dyn AbstractActiveService>) -> Self {
        Self { active_service }
    }

    pub fn router(self) -> OpenApiRouter {
        OpenApiRouter::new()
            .routes(routes!(get_active, update_active, delete_active))
            .routes(routes!(list_user_actives, create_active))
            .with_state(Arc::new(self))
    }
}

#[utoipa::path(
    get,
    path = "/{active_id}",
    tag = "active",
    params(
        ("active_id", description = "Active id"),
    ),
    responses(
        (status = 200, description = "Get active success", body = ActiveResponse)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn get_active(
    State(controller): State<Arc<ApiActiveController>>,
    Path(active_id): Path<u32>,
    Extension(user_id): Extension<u32>,
) -> Result<Json<ActiveResponse>, ApiError> {
    let active = controller
        .active_service
        .get_active(active_id)
        .await
        .map_err(ApiError::from)?;

    if active.user_id != user_id {
        return Err(ApiError::Forbidden(
            "Don't have access to active".to_string(),
        ));
    }

    let response = ActiveResponse::from(active);
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "",
    tag = "active",
    responses(
        (status = 200, description = "List active success", body = Vec<ActiveResponse>)
    ),
    security(("Authorization" = [])),
)]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn list_user_actives(
    State(controller): State<Arc<ApiActiveController>>,
    Extension(user_id): Extension<u32>,
) -> Result<Json<Vec<ActiveResponse>>, ApiError> {
    let actives = controller.active_service.list_user_actives(user_id).await?;

    let result = actives
        .into_iter()
        .map(ActiveResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "",
    tag = "active",
    responses(
        (status = 200, description = "Create active success", body = ActiveResponse)
    ),
    security(("Authorization" = [])),
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn create_active(
    State(controller): State<Arc<ApiActiveController>>,
    Extension(user_id): Extension<u32>,
    Json(active): Json<ActiveRequest>,
) -> Result<Json<ActiveResponse>, ApiError> {
    let mut entity: domain::models::ActiveEntity = active.try_into()?;
    entity.user_id = user_id;
    controller
        .active_service
        .create_active(entity)
        .await
        .map(ActiveResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    put,
    path = "/{active_id}",
    tag = "active",
    params(
        ("active_id", description = "Active id"),
    ),
    responses(
        (status = 200, description = "Update active success", body = ActiveResponse)
    ),
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn update_active(
    State(controller): State<Arc<ApiActiveController>>,
    Json(active): Json<ActiveRequest>,
) -> Result<Json<ActiveResponse>, ApiError> {
    let data = active.try_into()?;
    controller
        .active_service
        .update_active(data)
        .await
        .map(ActiveResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}

#[utoipa::path(
    delete,
    path = "/{active_id}",
    tag = "active",
    params(
        ("active_id", description = "Active id"),
    ),
    responses(
        (status = 200, description = "Delete active success", body = ActiveResponse)
    ),
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn delete_active(
    State(controller): State<Arc<ApiActiveController>>,
    Path(active_id): Path<u32>,
) -> Result<Json<ActiveResponse>, ApiError> {
    controller
        .active_service
        .delete_active(active_id)
        .await
        .map(ActiveResponse::from)
        .map(Json)
        .map_err(ApiError::from)
}
