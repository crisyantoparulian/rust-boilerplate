#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    use crate::models::User;
    use crate::services::{UserService, MockUserService, UserServiceFactory};
    use uuid::Uuid;

    fn create_test_app() -> Router {
        let user_service = crate::services::UserServiceFactory::create_user_service();
        crate::routes::create_routes().with_state(user_service)
    }

    fn create_mock_app_with_user() -> Router {
        let test_user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );
        let user_service = crate::services::UserServiceFactory::create_user_service_with_data(vec![test_user]);
        crate::routes::create_routes().with_state(user_service)
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = create_test_app();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let health_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(health_response["success"], true);
        assert_eq!(health_response["data"]["status"], "healthy");
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let app = create_test_app();

        let user_data = serde_json::json!({
            "email": "newuser@example.com",
            "password": "password123"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/users")
            .header("content-type", "application/json")
            .body(Body::from(user_data.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let user_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(user_response["success"], true);
        assert_eq!(user_response["data"]["email"], "newuser@example.com");
    }

    #[tokio::test]
    async fn test_create_user_validation_error() {
        let app = create_test_app();

        let user_data = serde_json::json!({
            "email": "invalid-email", // Invalid email
            "password": "123" // Too short
        });

        let request = Request::builder()
            .method("POST")
            .uri("/users")
            .header("content-type", "application/json")
            .body(Body::from(user_data.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(error_response["success"], false);
        assert_eq!(error_response["error"]["code"], "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let app = create_test_app();

        let request = Request::builder()
            .uri(&format!("/users/{}", Uuid::new_v4()))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(error_response["success"], false);
        assert_eq!(error_response["error"]["code"], "NOT_FOUND");
    }

    #[tokio::test]
    async fn test_list_users_empty() {
        let app = create_test_app();

        let request = Request::builder()
            .uri("/users")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let users_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(users_response["success"], true);
        assert_eq!(users_response["data"].as_array().unwrap().len(), 0);
        assert_eq!(users_response["meta"]["total"], 0);
    }

    #[tokio::test]
    async fn test_list_users_with_data() {
        let app = create_mock_app_with_user();

        let request = Request::builder()
            .uri("/users")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let users_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(users_response["success"], true);
        assert_eq!(users_response["data"].as_array().unwrap().len(), 1);
        assert_eq!(users_response["meta"]["total"], 1);
        assert_eq!(users_response["meta"]["page"], 1);
        assert_eq!(users_response["meta"]["limit"], 10);
    }
}