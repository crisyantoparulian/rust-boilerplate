use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::{Duration, Instant};
use tracing::{info, warn, error, debug, Instrument};
use uuid::Uuid;
use log;

/// Request logging middleware with correlation IDs and performance metrics
pub async fn request_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();

    // Generate correlation ID if not present
    let correlation_id = extract_or_generate_correlation_id(request.headers());

    // Create span for this request
    let span = tracing::info_span!(
        "http_request",
        correlation_id = %correlation_id,
        method = %method,
        uri = %uri,
        version = ?version,
    );

    // Log request details
    log_request_details(&request, &correlation_id);

    // Process request with span context
    async {
        let response = next.run(request).await;

        let duration = start_time.elapsed();
        let status = response.status();
        let status_code = status.as_u16();

        // Log response details
        log_response_details(&response, &correlation_id, duration, status_code);

        response
    }.instrument(span).await
}

/// Enhanced error logging middleware
pub async fn error_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let correlation_id = extract_or_generate_correlation_id(request.headers());
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    let status = response.status();
    if status.is_server_error() {
        error!(
            correlation_id = correlation_id,
            method = %method,
            uri = %uri,
            status_code = status.as_u16(),
            "Server error occurred during request processing"
        );
    } else if status.is_client_error() && status.as_u16() >= 400 {
        warn!(
            correlation_id = correlation_id,
            method = %method,
            uri = %uri,
            status_code = status.as_u16(),
            "Client error occurred during request processing"
        );
    }

    response
}

/// Security logging middleware for suspicious activities
pub async fn security_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let headers = request.headers().clone();
    let uri = request.uri().clone();
    let method = request.method().clone();
    let correlation_id = extract_or_generate_correlation_id(&headers);

    // Log suspicious patterns
    detect_suspicious_activity(&headers, &uri, &method, &correlation_id);

    let response = next.run(request).await;

    // Log authentication failures
    if response.status() == StatusCode::UNAUTHORIZED {
        warn!(
            correlation_id = correlation_id,
            method = %method,
            uri = %uri,
            user_agent = get_header_value(&headers, "user-agent"),
            ip_address = get_client_ip(&headers),
            "Authentication failed"
        );
    }

    response
}

/// Extract correlation ID from headers or generate a new one
pub fn extract_or_generate_correlation_id(headers: &HeaderMap) -> String {
    // Try to extract from common header names
    const CORRELATION_HEADERS: [&str; 5] = [
        "x-correlation-id",
        "x-request-id",
        "x-trace-id",
        "request-id",
        "correlation-id",
    ];

    for header_name in CORRELATION_HEADERS {
        if let Some(correlation_id) = headers.get(header_name) {
            if let Ok(id_str) = correlation_id.to_str() {
                return id_str.to_string();
            }
        }
    }

    // Generate new correlation ID
    Uuid::new_v4().to_string()
}

/// Log request details with structured logging
fn log_request_details(request: &Request, correlation_id: &str) {
    let method = request.method();
    let uri = request.uri();
    let user_agent = get_header_value(request.headers(), "user-agent");
    let content_type = get_header_value(request.headers(), "content-type");
    let content_length = get_header_value(request.headers(), "content-length");

    // Log basic request info
    info!(
        correlation_id = correlation_id,
        method = %method,
        uri = %uri,
        user_agent = user_agent,
        content_type = content_type,
        content_length = content_length,
        "Incoming request"
    );

    // Log request body for POST/PUT requests (in debug mode only)
    if (method == axum::http::Method::POST || method == axum::http::Method::PUT)
        && log::log_enabled!(log::Level::Debug) {

        debug!(correlation_id = correlation_id, "Request body will be logged by individual handlers");
    }

    // Log query parameters
    if let Some(query) = uri.query() {
        debug!(
            correlation_id = correlation_id,
            query = query,
            "Request query parameters"
        );
    }
}

/// Log response details with performance metrics
fn log_response_details(
    response: &Response,
    correlation_id: &str,
    duration: Duration,
    status_code: u16,
) {
    let content_type = get_header_value(response.headers(), "content-type");
    let content_length = get_header_value(response.headers(), "content-length");

    // Determine log level based on status code
    match status_code {
        200..=299 => {
            info!(
                correlation_id = correlation_id,
                status_code = status_code,
                duration_ms = duration.as_millis(),
                content_type = content_type,
                content_length = content_length,
                "Request completed successfully"
            );
        }
        300..=399 => {
            info!(
                correlation_id = correlation_id,
                status_code = status_code,
                duration_ms = duration.as_millis(),
                content_type = content_type,
                "Request redirected"
            );
        }
        400..=499 => {
            warn!(
                correlation_id = correlation_id,
                status_code = status_code,
                duration_ms = duration.as_millis(),
                content_type = content_type,
                "Client error occurred"
            );
        }
        500..=599 => {
            error!(
                correlation_id = correlation_id,
                status_code = status_code,
                duration_ms = duration.as_millis(),
                content_type = content_type,
                "Server error occurred"
            );
        }
        _ => {
            warn!(
                correlation_id = correlation_id,
                status_code = status_code,
                duration_ms = duration.as_millis(),
                "Unknown status code"
            );
        }
    }

    // Log performance warnings
    let duration_ms = duration.as_millis();
    if duration_ms > 1000 {
        warn!(
            correlation_id = correlation_id,
            duration_ms = duration_ms,
            "Slow request detected (>1000ms)"
        );
    } else if duration_ms > 500 {
        info!(
            correlation_id = correlation_id,
            duration_ms = duration_ms,
            "Request took longer than expected (>500ms)"
        );
    }

    // Log response body size for large responses
    if let Some(content_length_str) = get_header_value(response.headers(), "content-length") {
        if let Ok(content_length_num) = content_length_str.parse::<usize>() {
            if content_length_num > 1_000_000 { // > 1MB
                info!(
                    correlation_id = correlation_id,
                    content_length = content_length_num,
                    "Large response detected (>1MB)"
                );
            }
        }
    }
}

/// Helper function to safely extract header values
fn get_header_value(headers: &HeaderMap, header_name: &str) -> Option<String> {
    headers
        .get(header_name)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Detect suspicious request patterns
fn detect_suspicious_activity(
    headers: &HeaderMap,
    uri: &axum::http::Uri,
    method: &axum::http::Method,
    correlation_id: &str,
) {
    // Check for suspicious user agents
    if let Some(user_agent) = get_header_value(headers, "user-agent") {
        let suspicious_agents = [
            "sqlmap", "nikto", "nmap", "masscan", "zap", "burp",
            "scanner", "crawler", "bot", "spider"
        ];

        for agent in suspicious_agents {
            if user_agent.to_lowercase().contains(&agent.to_string()) {
                warn!(
                    correlation_id = correlation_id,
                    user_agent = user_agent,
                    suspicious_pattern = agent,
                    "Suspicious user agent detected"
                );
            }
        }
    }

    // Check for suspicious URL patterns
    let uri_str = uri.to_string();
    let suspicious_patterns = [
        "..", "%2e%2e", "/etc/passwd", "/proc/self",
        "<script", "javascript:", "eval(", "alert(",
        "union select", "drop table", "insert into"
    ];

    for pattern in suspicious_patterns {
        if uri_str.to_lowercase().contains(&pattern.to_string()) {
            warn!(
                correlation_id = correlation_id,
                uri = uri_str,
                suspicious_pattern = pattern,
                method = %method,
                "Suspicious URL pattern detected"
            );
        }
    }

    // Check for large header sizes
    let header_size: usize = headers
        .iter()
        .map(|(name, value)| name.as_str().len() + value.len())
        .sum();

    if header_size > 8192 { // > 8KB
        warn!(
            correlation_id = correlation_id,
            header_size_bytes = header_size,
            "Unusually large headers detected"
        );
    }
}

/// Attempt to get client IP from headers
fn get_client_ip(headers: &HeaderMap) -> Option<String> {
    const IP_HEADERS: [&str; 5] = [
        "x-forwarded-for",
        "x-real-ip",
        "cf-connecting-ip",
        "x-client-ip",
        "x-forwarded"
    ];

    for header_name in IP_HEADERS {
        if let Some(ip_value) = headers.get(header_name) {
            if let Ok(ip_str) = ip_value.to_str() {
                // Take the first IP if multiple are present
                let ip = ip_str.split(',').next()?.trim();
                return Some(ip.to_string());
            }
        }
    }

    None
}

/// Request body logging for debugging (to be used in individual handlers)
pub fn log_request_body(correlation_id: &str, endpoint: &str, body: &str) {
    // Only log if debug level is enabled and body is not too large
    if log::log_enabled!(log::Level::Debug) && body.len() < 10000 {
        debug!(
            correlation_id = correlation_id,
            endpoint = endpoint,
            body_size = body.len(),
            body = body,
            "Request body details"
        );
    } else if log::log_enabled!(log::Level::Info) {
        info!(
            correlation_id = correlation_id,
            endpoint = endpoint,
            body_size = body.len(),
            "Request body received (too large for debug logging)"
        );
    }
}

/// Response body logging for debugging (to be used in individual handlers)
pub fn log_response_body(correlation_id: &str, endpoint: &str, body: &str) {
    // Only log if debug level is enabled and body is not too large
    if log::log_enabled!(log::Level::Debug) && body.len() < 10000 {
        debug!(
            correlation_id = correlation_id,
            endpoint = endpoint,
            body_size = body.len(),
            body = body,
            "Response body details"
        );
    } else if log::log_enabled!(log::Level::Info) {
        info!(
            correlation_id = correlation_id,
            endpoint = endpoint,
            body_size = body.len(),
            "Response body sent (too large for debug logging)"
        );
    }
}

