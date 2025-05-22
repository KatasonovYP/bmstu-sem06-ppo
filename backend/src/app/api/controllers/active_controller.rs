use std::sync::Arc;

use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};

use crate::{
    app::api::{
        api_errors::ApiError,
        dto::active_dto::{
            CreateActiveRequest,
            CreateActiveResponse,
            GetActiveResponse,
        },
    },
    domain::ports::domain::AbstractActiveService,
};

#[derive(Clone)]
pub struct ApiActiveController {
    active_service: Arc<dyn AbstractActiveService>,
}

impl ApiActiveController {
    pub fn new(active_service: Arc<dyn AbstractActiveService>) -> Self {
        Self { active_service }
    }

    #[tracing::instrument(skip(controller), err(Debug), ret)]
    pub async fn get_active(
        State(controller): State<Self>,
        Path(active_id): Path<u32>,
    ) -> Result<Json<GetActiveResponse>, ApiError> {
        controller
            .active_service
            .get_active(active_id)
            .await
            .map(GetActiveResponse::from)
            .map(Json)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(controller), err(Debug), ret)]
    pub async fn create_active(
        State(controller): State<Self>,
        Json(new_active): Json<CreateActiveRequest>,
    ) -> Result<Json<CreateActiveResponse>, ApiError> {
        let active_data = new_active.try_into()?;

        controller
            .active_service
            .create_active(active_data)
            .await
            .map(GetActiveResponse::from)
            .map(Json)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(controller), err(Debug), ret)]
    pub async fn list_actives(
        State(controller): State<Self>,
    ) -> Result<Json<Vec<GetActiveResponse>>, ApiError> {
        let users = controller.active_service.list_actives().await?;

        let result = users
            .into_iter()
            .map(GetActiveResponse::from)
            .collect::<Vec<_>>();

        Ok(Json(result))
    }

    #[tracing::instrument(skip(controller), err(Debug), ret)]
    pub async fn list_user_actives(
        State(controller): State<Self>,
        Path(user_id): Path<u32>,
    ) -> Result<Json<Vec<GetActiveResponse>>, ApiError> {
        let users = controller.active_service.list_user_actives(user_id).await?;

        let result = users
            .into_iter()
            .map(GetActiveResponse::from)
            .collect::<Vec<_>>();

        Ok(Json(result))
    }
}
