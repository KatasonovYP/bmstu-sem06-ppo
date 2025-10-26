use thiserror::Error;

#[derive(Clone, Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Entity not found: {entity} with id {id}")]
    EntityNotFound { entity: String, id: String },

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
