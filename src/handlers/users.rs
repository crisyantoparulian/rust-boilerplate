use axum::{extract::{Path, State}, response::{Response, IntoResponse}, Json};
use uuid::Uuid;
use validator::Validate;

use crate::models::{CreateUserRequest, UserResponse};
use crate::response::{
    Meta, success_response, success_response_with_meta,
    validation_error_response, not_found_response, bad_request_response
};
use crate::services::UserService;
use crate::middleware;

/// Create a new user
pub async fn create_user(
    State(user_service): State<std::sync::Arc<dyn UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Response, Response> {
    // Log request body in debug mode (using fixed correlation ID for now)
    let correlation_id = uuid::Uuid::new_v4().to_string();
    if let Ok(body_str) = serde_json::to_string(&payload) {
        middleware::log_request_body(&correlation_id, "create_user", &body_str);
    }

    // Validate request
    if let Err(validation_errors) = payload.validate() {
        let errors: Vec<String> = validation_errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!("{}: {}", field, error.message.as_ref().unwrap_or(&"Invalid value".into()))
                })
            })
            .collect();

        return Err(validation_error_response(errors).into_response());
    }

    // Create user via service
    match user_service.create_user(payload).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                created_at: user.created_at,
            };
            Ok(success_response(response).into_response())
        }
        Err(crate::services::ServiceError::AlreadyExists) => {
            Err(bad_request_response("User with this email already exists").into_response())
        }
        Err(crate::services::ServiceError::Validation(msg)) => {
            Err(bad_request_response(&msg).into_response())
        }
        Err(_) => {
            Err(crate::response::internal_error_response("Failed to create user").into_response())
        }
    }
}

/// Get user by ID
pub async fn get_user(
    State(user_service): State<std::sync::Arc<dyn UserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Response, Response> {
    match user_service.get_user_by_id(user_id).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                created_at: user.created_at,
            };
            Ok(success_response(response).into_response())
        }
        Err(crate::services::ServiceError::NotFound) => {
            Err(not_found_response("User").into_response())
        }
        Err(_) => {
            Err(crate::response::internal_error_response("Failed to retrieve user").into_response())
        }
    }
}

/// List users with pagination
pub async fn list_users(
    State(user_service): State<std::sync::Arc<dyn UserService>>,
    axum::extract::Query(params): axum::extract::Query<ListUsersParams>,
) -> Result<Response, Response> {
    // Validate pagination parameters
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).min(100).max(1);

    match user_service.list_users(page, limit).await {
        Ok((users, total)) => {
            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: user.id,
                    email: user.email,
                    created_at: user.created_at,
                })
                .collect();

            let meta = Meta::new(page, limit, total);
            Ok(success_response_with_meta(user_responses, meta).into_response())
        }
        Err(_) => {
            Err(crate::response::internal_error_response("Failed to list users").into_response())
        }
    }
}

/// Update user (placeholder for future implementation)
pub async fn update_user(
    State(user_service): State<std::sync::Arc<dyn UserService>>,
    Path(user_id): Path<Uuid>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Response, Response> {
    // Check if user exists first
    match user_service.get_user_by_id(user_id).await {
        Ok(_user) => {
            // TODO: Implement update logic
            Err(crate::response::bad_request_response("Update functionality not implemented yet").into_response())
        }
        Err(crate::services::ServiceError::NotFound) => {
            Err(not_found_response("User").into_response())
        }
        Err(_) => {
            Err(crate::response::internal_error_response("Failed to update user").into_response())
        }
    }
}

/// Delete user (placeholder for future implementation)
pub async fn delete_user(
    State(user_service): State<std::sync::Arc<dyn UserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Response, Response> {
    // Check if user exists first
    match user_service.get_user_by_id(user_id).await {
        Ok(_user) => {
            // TODO: Implement delete logic
            Err(crate::response::bad_request_response("Delete functionality not implemented yet").into_response())
        }
        Err(crate::services::ServiceError::NotFound) => {
            Err(not_found_response("User").into_response())
        }
        Err(_) => {
            Err(crate::response::internal_error_response("Failed to delete user").into_response())
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ListUsersParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}