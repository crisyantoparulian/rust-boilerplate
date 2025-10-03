use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::entities::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError>;
    async fn list(&self, page: u32, limit: u32) -> Result<(Vec<User>, u64), RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Internal error: {0}")]
    Internal(String),
}