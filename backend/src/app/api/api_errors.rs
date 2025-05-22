use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use schemars::JsonSchema;
use serde_json::json;

use crate::domain::errors::DomainError;

#[derive(Clone, Debug, JsonSchema)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::EntityNotFound { entity, id } => {
                ApiError::NotFound(format!("Entity not found: {entity} with id {id}"))
            },
            DomainError::ValidationError(msg) => ApiError::BadRequest(msg),
            _ => ApiError::InternalError(err.to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
