mod config;
mod error;
mod middleware;
mod response;
mod domain;
mod infrastructure;
mod delivery;
mod container;

use config::Config;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize tracing using infrastructure logger
    infrastructure::init_logger();

    // Load configuration
    let config = Config::from_env();
    tracing::info!("Starting server at {}:{}", config.server_host, config.server_port);

    // Create router with clean architecture layers
    let app = delivery::create_routes()
        // Apply logging middleware layers
        .layer(axum::middleware::from_fn(middleware::security_logging_middleware))
        .layer(axum::middleware::from_fn(middleware::error_logging_middleware))
        .layer(axum::middleware::from_fn(middleware::request_logging_middleware))
        // Add HTTP tracing layer for distributed tracing
        .layer(tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                let correlation_id = middleware::extract_or_generate_correlation_id(request.headers());
                tracing::info_span!(
                    "http_request",
                    correlation_id = %correlation_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
        );

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
        .await?;

    tracing::info!("Server listening on {}:{}", config.server_host, config.server_port);
    tracing::info!("Available endpoints:");
    tracing::info!("  GET  /api/health     - Health check");
    tracing::info!("  GET  /api/ready      - Readiness check");
    tracing::info!("  GET  /api/live       - Liveness check");
    tracing::info!("  GET  /api/users      - List users (with pagination)");
    tracing::info!("  POST /api/users      - Create user");
    tracing::info!("  GET  /api/users/:id  - Get user by ID");
    tracing::info!("  PUT  /api/users/:id  - Update user (placeholder)");
    tracing::info!("  DELETE /api/users/:id - Delete user (placeholder)");

    axum::serve(listener, app).await?;

    Ok(())
}