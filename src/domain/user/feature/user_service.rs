use async_trait::async_trait;
use std::sync::Arc;
use validator::Validate;
use crate::domain::user::entities::User;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::model::{CreateUserRequest, UserResponse, ListUsersRequest, ListUsersResponse};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, ServiceError>;
    async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<UserResponse>, ServiceError>;
    async fn list_users(&self, request: ListUsersRequest) -> Result<ListUsersResponse, ServiceError>;
}

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, ServiceError> {
        // Validate request
        if let Err(validation_errors) = request.validate() {
            let errors: Vec<String> = validation_errors
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!("{}: {}", field, error.message.as_ref().unwrap_or(&"Invalid value".into()))
                    })
                })
                .collect();
            return Err(ServiceError::Validation(errors.join(", ")));
        }

        // Check if user already exists
        if self.repository.exists_by_email(&request.email).await? {
            return Err(ServiceError::AlreadyExists);
        }

        // Create new user with password hashing
        let password_hash = format!("hashed_{}", request.password); // Simplified hashing
        let user = User::new(request.email, password_hash);

        // Save user
        self.repository.save(&user).await?;

        Ok(UserResponse::from(user))
    }

    async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<UserResponse>, ServiceError> {
        match self.repository.find_by_id(id).await? {
            Some(user) => Ok(Some(UserResponse::from(user))),
            None => Ok(None),
        }
    }

    async fn list_users(&self, request: ListUsersRequest) -> Result<ListUsersResponse, ServiceError> {
        let page = request.page.unwrap_or(1).max(1);
        let limit = request.limit.unwrap_or(10).min(100).max(1);

        let (users, total) = self.repository.list(page, limit).await?;
        let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

        Ok(ListUsersResponse {
            users: user_responses,
            total,
            page,
            limit,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Repository error: {0}")]
    Repository(#[from] crate::domain::user::repository::RepositoryError),
}