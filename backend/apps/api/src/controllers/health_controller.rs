use std::sync::Arc;

use axum::{
    Json,
    http::StatusCode,
};
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::api_errors::ApiError;

#[derive(Clone)]
pub struct ApiHealthController {}

impl ApiHealthController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn router(self) -> OpenApiRouter {
        OpenApiRouter::new()
            .routes(routes!(get_ping))
            .routes(routes!(get_version))
            .with_state(Arc::new(self))
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct PingResponse {
    status_code: u16,
}

#[utoipa::path(
        get,
        path = "/ping",
        tag = "health",
        description = "Проверяет доступность приложения",
        responses(
            (status = 200, description = "Ping successful", body = PingResponse),
        )
    )]
#[axum_macros::debug_handler]
#[tracing::instrument(err(Debug), ret)]
async fn get_ping() -> Result<Json<PingResponse>, ApiError> {
    Ok(Json(PingResponse {
        status_code: StatusCode::OK.as_u16(),
    }))
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct VersionResponse {
    version: String,
}

#[utoipa::path(
        get,
        path = "/version",
        tag = "health",
        description = "Возвращает версию приложения",
        responses(
            (status = 200, description = "Version successful", body = VersionResponse),
        )
    )]
#[axum_macros::debug_handler]
#[tracing::instrument(err(Debug), ret)]
async fn get_version() -> Result<Json<VersionResponse>, ApiError> {
    Ok(Json(VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

#[axum_macros::debug_handler]
#[tracing::instrument(err(Debug), ret)]
pub async fn not_found_handler() -> Result<Json<PingResponse>, ApiError> {
    Err(ApiError::NotFound("Path not exists".to_string()))
}
