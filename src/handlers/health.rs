use axum::{extract::State, response::IntoResponse};
use crate::response::success_response;
use crate::services::UserService;

#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub database_status: String,
}

pub async fn health_check(
    State(user_service): State<std::sync::Arc<dyn UserService>>,
) -> Result<impl axum::response::IntoResponse, axum::response::Response> {
    // Check database connectivity by testing service
    let database_status = match user_service.list_users(1, 1).await {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };

    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_status,
    };

    Ok(success_response(response))
}

pub async fn readiness_check() -> Result<impl axum::response::IntoResponse, axum::response::Response> {
    let response = serde_json::json!({
        "status": "ready",
        "timestamp": chrono::Utc::now()
    });

    Ok(success_response(response))
}

pub async fn liveness_check() -> Result<impl axum::response::IntoResponse, axum::response::Response> {
    let response = serde_json::json!({
        "status": "alive",
        "timestamp": chrono::Utc::now()
    });

    Ok(success_response(response))
}