use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::User;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("User not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("User already exists")]
    AlreadyExists,
}

pub struct UserRepository {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create(&self, user: &User) -> Result<(), DatabaseError> {
        let mut users = self.users.write().await;

        // Check if user already exists by email
        for existing_user in users.values() {
            if existing_user.email == user.email {
                return Err(DatabaseError::AlreadyExists);
            }
        }

        users.insert(user.id, user.clone());
        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DatabaseError> {
        let users = self.users.read().await;
        Ok(users.get(&id).cloned())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, DatabaseError> {
        let users = self.users.read().await;
        Ok(users.values().find(|user| user.email == email).cloned())
    }
}