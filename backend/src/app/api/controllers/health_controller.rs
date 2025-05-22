use aide::OperationOutput;
use axum::{
    http::StatusCode,
    Json,
};

use crate::app::api::api_errors::ApiError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PingResponse {
    status_code: u16,
}

#[derive(Clone)]
pub struct ApiHealthController {}

impl ApiHealthController {
    #[tracing::instrument(ret)]
    pub async fn get_ping() -> (StatusCode, Result<Json<PingResponse>, ApiError>) {
        (
            StatusCode::CREATED,
            Ok(Json(PingResponse {
                status_code: StatusCode::OK.as_u16(),
            })),
        )
    }
}

impl OperationOutput for PingResponse {
    type Inner = Self;
}
