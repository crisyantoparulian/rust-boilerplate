use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub service: String,
}

impl HealthResponse {
    pub fn healthy(service: String) -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            service,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
}