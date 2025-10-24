use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use domain::errors::DomainError;
use serde_json::json;

#[derive(Clone, Debug, utoipa::ToSchema)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    NoContent(String),
    Forbidden(String),
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
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::NoContent(msg) => (StatusCode::NO_CONTENT, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
