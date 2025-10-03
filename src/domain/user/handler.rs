use axum::{
    extract::{Path, State, Query},
    response::{Response, IntoResponse},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use super::feature::UserService;
use super::model::{CreateUserRequest, ListUsersRequest};
use crate::response::{success_response, not_found_response, bad_request_response};

pub async fn create_user(
    State(user_service): State<Arc<dyn UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Response, Response> {
    // Log request body in debug mode
    let correlation_id = uuid::Uuid::new_v4().to_string();
    if let Ok(body_str) = serde_json::to_string(&payload) {
        crate::middleware::log_request_body(&correlation_id, "create_user", &body_str);
    }

    match user_service.create_user(payload).await {
        Ok(user_response) => Ok(success_response(user_response).into_response()),
        Err(super::feature::ServiceError::AlreadyExists) => {
            Err(bad_request_response("User with this email already exists").into_response())
        }
        Err(super::feature::ServiceError::Validation(msg)) => {
            Err(bad_request_response(&msg).into_response())
        }
        Err(_) => {
            Err(crate::response::internal_error_response("Failed to create user").into_response())
        }
    }
}

pub async fn get_user(
    State(user_service): State<Arc<dyn UserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Response, Response> {
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user_response)) => Ok(success_response(user_response).into_response()),
        Ok(None) => Err(not_found_response("User").into_response()),
        Err(_) => Err(crate::response::internal_error_response("Failed to retrieve user").into_response()),
    }
}

pub async fn list_users(
    State(user_service): State<Arc<dyn UserService>>,
    Query(params): Query<ListUsersParams>,
) -> Result<Response, Response> {
    let request = ListUsersRequest {
        page: params.page,
        limit: params.limit,
    };

    match user_service.list_users(request).await {
        Ok(response) => Ok(success_response(response).into_response()),
        Err(_) => Err(crate::response::internal_error_response("Failed to list users").into_response()),
    }
}

pub async fn update_user(
    State(user_service): State<Arc<dyn UserService>>,
    Path(user_id): Path<Uuid>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Response, Response> {
    // Check if user exists first
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(_user)) => {
            Err(bad_request_response("Update functionality not implemented yet").into_response())
        }
        Ok(None) => Err(not_found_response("User").into_response()),
        Err(_) => Err(crate::response::internal_error_response("Failed to update user").into_response()),
    }
}

pub async fn delete_user(
    State(user_service): State<Arc<dyn UserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Response, Response> {
    // Check if user exists first
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(_user)) => {
            Err(bad_request_response("Delete functionality not implemented yet").into_response())
        }
        Ok(None) => Err(not_found_response("User").into_response()),
        Err(_) => Err(crate::response::internal_error_response("Failed to delete user").into_response()),
    }
}

#[derive(serde::Deserialize)]
pub struct ListUsersParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}