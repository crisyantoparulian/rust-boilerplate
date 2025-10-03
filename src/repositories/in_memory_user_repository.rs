use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::entities::{User, CreateUserRequest};
use crate::domain::repositories::UserRepository;
use crate::domain::errors::{DomainError, DomainResult};

pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, request: CreateUserRequest) -> DomainResult<User> {
        let mut users = self.users.write().await;

        // Check if user already exists by email
        for existing_user in users.values() {
            if existing_user.email == request.email {
                return Err(DomainError::UserAlreadyExists);
            }
        }

        // Create new user
        let password_hash = format!("hashed_{}", request.password); // Simplified hashing
        let user = User::new(request.email, password_hash);

        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>> {
        let users = self.users.read().await;
        Ok(users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        let users = self.users.read().await;
        Ok(users
            .values()
            .find(|user| user.email == email)
            .cloned())
    }

    async fn find_all(&self, page: u32, limit: u32) -> DomainResult<(Vec<User>, u64)> {
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

    async fn exists_by_email(&self, email: &str) -> DomainResult<bool> {
        let users = self.users.read().await;
        Ok(users.values().any(|user| user.email == email))
    }
}

impl InMemoryUserRepository {
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