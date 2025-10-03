use std::sync::Arc;
use crate::domain::user::feature::UserService;
use crate::domain::user::feature::UserServiceImpl;
use crate::domain::user::repository::InMemoryUserRepository;

pub struct AppContainer {
    pub user_service: Arc<dyn UserService>,
}

impl AppContainer {
    pub fn new() -> Self {
        // Create repository instances
        let user_repository = Arc::new(InMemoryUserRepository::new());

        // Create service instances with their dependencies
        let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repository));

        Self {
            user_service,
        }
    }
}

impl Default for AppContainer {
    fn default() -> Self {
        Self::new()
    }
}