use axum::Router;
use crate::domain::user::handler as user_handlers;
use crate::domain::health::handler as health_handlers;
use crate::container::AppContainer;

pub fn create_routes() -> Router {
    // Create dependency injection container
    let container = AppContainer::new();

    Router::new()
        // API routes with /api prefix
        .nest("/api", Router::new()
            // Health checks
            .route("/health", axum::routing::get(health_handlers::health_check))
            .route("/ready", axum::routing::get(health_handlers::readiness_check))
            .route("/live", axum::routing::get(health_handlers::liveness_check))

            // User endpoints
            .route("/users", axum::routing::post(user_handlers::create_user))
            .route("/users", axum::routing::get(user_handlers::list_users))
            .route("/users/:id", axum::routing::get(user_handlers::get_user))
            .route("/users/:id", axum::routing::put(user_handlers::update_user))
            .route("/users/:id", axum::routing::delete(user_handlers::delete_user))
        )

        // Provide user service as state from the container
        .with_state(container.user_service)
}