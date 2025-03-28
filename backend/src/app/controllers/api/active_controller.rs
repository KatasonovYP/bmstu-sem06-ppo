use std::sync::Arc;

use super::dto::active_dto::{CreateActiveRequest, CreateActiveResponse, GetActiveResponse};
use crate::ports::inbound::domain::AbstractActiveService;
use axum::{
    extract::{Path, State},
    Json,
};

#[derive(Clone)]
pub struct ActiveController {
    active_service: Arc<dyn AbstractActiveService>,
}

impl ActiveController {
    pub fn new(active_service: Arc<dyn AbstractActiveService>) -> Self {
        Self { active_service }
    }

    pub async fn get_active(
        State(controller): State<Self>,
        Path(active_id): Path<u32>,
    ) -> Json<GetActiveResponse> {
        controller
            .active_service
            .get_active(active_id)
            .await
            .map(GetActiveResponse::from)
            .map(Json)
            .unwrap()
    }
    pub async fn create_active(
        State(controller): State<Self>,
        Json(new_active): Json<CreateActiveRequest>,
    ) -> Json<CreateActiveResponse> {
        controller
            .active_service
            .create_active(new_active.into())
            .await
            .map(CreateActiveResponse::from)
            .map(Json)
            .unwrap()
    }
}
