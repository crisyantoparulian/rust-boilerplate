use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{User, CreateUserRequest, UserResponse};
use crate::response::{ApiResponse, Meta, ResponseSuccess};

/// Service trait for user operations - enables easy mocking and testing
#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, ServiceError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, ServiceError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, ServiceError>;
    async fn list_users(&self, page: u32, limit: u32) -> Result<(Vec<User>, u64), ServiceError>;
}

/// Standard service errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// In-memory implementation of UserService for testing and development
pub struct InMemoryUserService {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl InMemoryUserService {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn new_with_users(users: Vec<User>) -> Self {
        let user_map = users.into_iter().map(|user| (user.id, user)).collect();
        Self {
            users: Arc::new(RwLock::new(user_map)),
        }
    }
}

#[async_trait]
impl UserService for InMemoryUserService {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, ServiceError> {
        let mut users = self.users.write().await;

        // Check if user already exists by email
        for existing_user in users.values() {
            if existing_user.email == request.email {
                return Err(ServiceError::AlreadyExists);
            }
        }

        // Create new user
        let password_hash = format!("hashed_{}", request.password); // Simplified hashing
        let user = User::new(request.email, password_hash);

        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, ServiceError> {
        let users = self.users.read().await;
        users
            .get(&id)
            .cloned()
            .ok_or(ServiceError::NotFound)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, ServiceError> {
        let users = self.users.read().await;
        users
            .values()
            .find(|user| user.email == email)
            .cloned()
            .ok_or(ServiceError::NotFound)
    }

    async fn list_users(&self, page: u32, limit: u32) -> Result<(Vec<User>, u64), ServiceError> {
        let users = self.users.read().await;
        let user_list: Vec<User> = users.values().cloned().collect();
        let total = user_list.len() as u64;

        let offset = ((page - 1) * limit) as usize;
        let end = std::cmp::min(offset + limit as usize, user_list.len());

        if offset >= user_list.len() {
            return Ok((vec![], total));
        }

        let paginated_users = user_list[offset..end].to_vec();
        Ok((paginated_users, total))
    }
}

/// Mock implementation for testing
#[cfg(test)]
pub struct MockUserService {
    users: HashMap<Uuid, User>,
}

#[cfg(test)]
impl MockUserService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn with_user(mut self, user: User) -> Self {
        self.users.insert(user.id, user);
        self
    }

    pub fn should_fail_on_create(&mut self) -> &mut Self {
        // This would be used in tests to simulate failures
        self
    }
}

#[cfg(test)]
#[async_trait]
impl UserService for MockUserService {
    async fn create_user(&self, _request: CreateUserRequest) -> Result<User, ServiceError> {
        Err(ServiceError::Internal("Mock error".to_string()))
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, ServiceError> {
        self.users
            .get(&id)
            .cloned()
            .ok_or(ServiceError::NotFound)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, ServiceError> {
        self.users
            .values()
            .find(|user| user.email == email)
            .cloned()
            .ok_or(ServiceError::NotFound)
    }

    async fn list_users(&self, page: u32, limit: u32) -> Result<(Vec<User>, u64), ServiceError> {
        let user_list: Vec<User> = self.users.values().cloned().collect();
        let total = user_list.len() as u64;

        let offset = ((page - 1) * limit) as usize;
        let end = std::cmp::min(offset + limit as usize, user_list.len());

        if offset >= user_list.len() {
            return Ok((vec![], total));
        }

        let paginated_users = user_list[offset..end].to_vec();
        Ok((paginated_users, total))
    }
}

/// Service factory for easy dependency injection
pub struct ServiceFactory;

impl ServiceFactory {
    pub fn create_user_service() -> Arc<dyn UserService> {
        Arc::new(InMemoryUserService::new())
    }

    #[cfg(test)]
    pub fn create_mock_user_service() -> Arc<dyn UserService> {
        Arc::new(MockUserService::new())
    }

    pub fn create_user_service_with_data(users: Vec<User>) -> Arc<dyn UserService> {
        Arc::new(InMemoryUserService::new_with_users(users))
    }
}

// Re-exports
pub use ServiceFactory as UserServiceFactory;