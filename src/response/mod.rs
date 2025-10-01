use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

/// Standard API Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: Option<Meta>,
}

/// Metadata for paginated responses
#[derive(Debug, Serialize)]
pub struct Meta {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub total: Option<u64>,
    pub total_pages: Option<u32>,
}

impl Meta {
    pub fn new(page: u32, limit: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        Self {
            page: Some(page),
            limit: Some(limit),
            total: Some(total),
            total_pages: Some(total_pages),
        }
    }
}

/// Standard error structure
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }
    }
}

/// Trait for creating successful responses
pub trait ResponseSuccess<T: Serialize> {
    fn success(data: T) -> Self;
    fn success_with_meta(data: T, meta: Meta) -> Self;
}

/// Trait for creating error responses
pub trait ResponseError {
    fn error(error: ApiError) -> Self;
}

impl<T: Serialize> ResponseSuccess<T> for ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: None,
        }
    }

    fn success_with_meta(data: T, meta: Meta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(meta),
        }
    }
}

impl ResponseError for ApiResponse<()> {
    fn error(error: ApiError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            meta: None,
        }
    }
}

/// Helper functions for creating responses
pub mod helpers {
    use super::*;
    use crate::response::ApiResponse;

    pub fn success_response<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
        Json(ApiResponse::success(data))
    }

    pub fn success_response_with_meta<T: Serialize>(data: T, meta: Meta) -> Json<ApiResponse<T>> {
        Json(ApiResponse::success_with_meta(data, meta))
    }

    pub fn error_response(
        status: StatusCode,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> (StatusCode, Json<ApiResponse<()>>) {
        let error = ApiError::new(code, message);
        let response = ApiResponse::error(error);
        (status, Json(response))
    }

    pub fn error_response_with_details(
        status: StatusCode,
        code: impl Into<String>,
        message: impl Into<String>,
        details: HashMap<String, serde_json::Value>,
    ) -> (StatusCode, Json<ApiResponse<()>>) {
        let error = ApiError::with_details(code, message, details);
        let response = ApiResponse::error(error);
        (status, Json(response))
    }

    pub fn validation_error_response(
        validation_errors: Vec<String>,
    ) -> (StatusCode, Json<ApiResponse<()>>) {
        let mut details = HashMap::new();
        details.insert(
            "validation_errors".to_string(),
            json!(validation_errors).into(),
        );

        let error = ApiError::with_details(
            "VALIDATION_ERROR",
            "Request validation failed",
            details,
        );
        let response = ApiResponse::error(error);
        (StatusCode::BAD_REQUEST, Json(response))
    }

    pub fn not_found_response(resource: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        error_response(StatusCode::NOT_FOUND, "NOT_FOUND", format!("{} not found", resource))
    }

    pub fn internal_error_response(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message)
    }

    pub fn bad_request_response(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        error_response(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)
    }

    pub fn unauthorized_response(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        error_response(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", message)
    }
}

/// Implementation of IntoResponse for ApiResponse
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            match self.error.as_ref().map(|e| e.code.as_str()) {
                Some("BAD_REQUEST") => StatusCode::BAD_REQUEST,
                Some("UNAUTHORIZED") => StatusCode::UNAUTHORIZED,
                Some("FORBIDDEN") => StatusCode::FORBIDDEN,
                Some("NOT_FOUND") => StatusCode::NOT_FOUND,
                Some("VALIDATION_ERROR") => StatusCode::BAD_REQUEST,
                Some("CONFLICT") => StatusCode::CONFLICT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };

        (status, Json(self)).into_response()
    }
}

// Re-exports
pub use helpers::*;