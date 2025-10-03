use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::user::entities::User> for UserResponse {
    fn from(user: crate::domain::user::entities::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<UserResponse>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}