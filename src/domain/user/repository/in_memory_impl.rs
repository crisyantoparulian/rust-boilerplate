use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::user::entities::User;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::repository::RepositoryError;
use super::save;
use super::find_by_id;
use super::find_by_email;
use super::exists_by_email;
use super::list;

pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
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

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<(), RepositoryError> {
        save::save_user(self.users.clone(), user).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError> {
        find_by_id::find_user_by_id(self.users.clone(), id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        find_by_email::find_user_by_email(self.users.clone(), email).await
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError> {
        exists_by_email::user_exists_by_email(self.users.clone(), email).await
    }

    async fn list(&self, page: u32, limit: u32) -> Result<(Vec<User>, u64), RepositoryError> {
        list::list_users(self.users.clone(), page, limit).await
    }
}