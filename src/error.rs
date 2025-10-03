use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}


impl From<crate::domain::user::repository::RepositoryError> for AppError {
    fn from(err: crate::domain::user::repository::RepositoryError) -> Self {
        match err {
            crate::domain::user::repository::RepositoryError::NotFound => AppError::NotFound,
            crate::domain::user::repository::RepositoryError::AlreadyExists => AppError::BadRequest(err.to_string()),
            crate::domain::user::repository::RepositoryError::Database(msg) => AppError::Internal(msg),
            crate::domain::user::repository::RepositoryError::Internal(msg) => AppError::Internal(msg),
        }
    }
}