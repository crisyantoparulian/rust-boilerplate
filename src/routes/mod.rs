use axum::Router;
use crate::handlers;
use crate::services::UserServiceFactory;

pub fn create_routes() -> Router {
    // Create user service
    let user_service = UserServiceFactory::create_user_service();

    Router::new()
        // API routes with /api prefix
        .nest("/api", Router::new()
            // Health checks
            .route("/health", axum::routing::get(handlers::health_check))
            .route("/ready", axum::routing::get(handlers::readiness_check))
            .route("/live", axum::routing::get(handlers::liveness_check))

            // User endpoints
            .route("/users", axum::routing::post(handlers::create_user))
            .route("/users", axum::routing::get(handlers::list_users))
            .route("/users/:id", axum::routing::get(handlers::get_user))
            .route("/users/:id", axum::routing::put(handlers::update_user))
            .route("/users/:id", axum::routing::delete(handlers::delete_user))
        )

        // Provide user service as state
        .with_state(user_service)
}