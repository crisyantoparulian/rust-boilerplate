use axum::response::{Response, IntoResponse};
use super::model::{HealthResponse, ReadyResponse, LiveResponse, HealthCheck};
use crate::response::success_response;

pub async fn health_check() -> Response {
    let response = HealthResponse::healthy("rust-boilerplate".to_string());
    success_response(response).into_response()
}

pub async fn readiness_check() -> Response {
    let response = ReadyResponse {
        status: "ready".to_string(),
        timestamp: chrono::Utc::now(),
        checks: vec![
            HealthCheck {
                name: "database".to_string(),
                status: "healthy".to_string(),
            },
            HealthCheck {
                name: "memory".to_string(),
                status: "healthy".to_string(),
            },
        ],
    };
    success_response(response).into_response()
}

pub async fn liveness_check() -> Response {
    let response = LiveResponse {
        status: "alive".to_string(),
        timestamp: chrono::Utc::now(),
    };
    success_response(response).into_response()
}