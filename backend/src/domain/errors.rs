use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Entity not found: {entity} with id {id}")]
    EntityNotFound { entity: String, id: String },

    // #[error("Authorization error: {0}")]
    // AuthorizationError(String),

    // #[error("Business rule violation: {0}")]
    // BusinessRuleViolation(String),

    // #[error("Conflict: {0}")]
    // Conflict(String),
    #[error("External service error: {service} - {message}")]
    ExternalServiceError { service: String, message: String },

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
