# Rust HTTP API Boilerplate with Standardized JSON Responses

A clean, production-ready Rust HTTP API boilerplate with standardized JSON responses, clean architecture, and easy mocking for testing.

## 🚀 Features

- **Standardized JSON Responses**: Consistent API response format for success and error cases
- **Clean Architecture**: Separation of concerns with service layer and dependency injection
- **Easy Mocking**: Trait-based services for effortless testing and mocking
- **Validation**: Request validation with detailed error messages
- **Structured Logging**: Comprehensive logging with tracing
- **Health Checks**: Multiple health check endpoints for monitoring
- **Pagination**: Built-in pagination support for list endpoints
- **Async/Await**: Full async support with Tokio runtime

## 📁 Project Structure

```
rust-boilerplate/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── response/            # Standardized response system
│   │   └── mod.rs          # ApiResponse, error handling, helpers
│   ├── models/              # Data models and DTOs
│   │   ├── user.rs         # User model and validation
│   │   └── mod.rs
│   ├── services/            # Business logic layer
│   │   └── mod.rs          # UserService trait and implementations
│   ├── handlers/            # HTTP request handlers
│   │   ├── health.rs       # Health check endpoints
│   │   ├── users.rs        # User CRUD operations
│   │   └── mod.rs
│   ├── routes/              # Route definitions
│   │   └── mod.rs
│   ├── database/            # Data access layer (legacy)
│   │   └── mod.rs
│   ├── error.rs             # Error definitions (legacy)
│   └── tests/               # Integration tests
│       └── mod.rs
├── .env.example            # Environment variables template
├── Cargo.toml              # Dependencies and configuration
└── README.md              # This file
```

## 🏗️ Architecture

### Standard JSON Response Format

All API responses follow this consistent format:

#### Success Response
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "created_at": "2023-01-01T00:00:00Z"
  },
  "error": null,
  "meta": null
}
```

#### Paginated Response
```json
{
  "success": true,
  "data": [...],
  "error": null,
  "meta": {
    "page": 1,
    "limit": 10,
    "total": 100,
    "total_pages": 10
  }
}
```

#### Error Response
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Request validation failed",
    "details": {
      "validation_errors": ["email: Invalid email format"]
    }
  },
  "meta": null
}
```

### Service Layer Pattern

The application uses a clean service layer pattern with trait-based interfaces:

```rust
#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, ServiceError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, ServiceError>;
    async fn list_users(&self, page: u32, limit: u32) -> Result<(Vec<User>, u64), ServiceError>;
}
```

This pattern enables:
- **Easy Testing**: Mock implementations for unit tests
- **Dependency Injection**: Clean separation of concerns
- **Flexibility**: Easy to swap implementations

## 🚦 Available Endpoints

### Health Checks
- `GET /api/health` - Comprehensive health check including database
- `GET /api/ready` - Readiness probe for container orchestration
- `GET /api/live` - Liveness probe for container orchestration

### User Management
- `POST /api/users` - Create a new user
- `GET /api/users` - List users with pagination
- `GET /api/users/:id` - Get user by ID
- `PUT /api/users/:id` - Update user (placeholder)
- `DELETE /api/users/:id` - Delete user (placeholder)

## 🛠️ Quick Start

1. **Clone and Run**
   ```bash
   cargo run
   ```

2. **Test Health Check**
   ```bash
   curl http://localhost:3000/api/health
   ```

3. **Create a User**
   ```bash
   curl -X POST http://localhost:3000/api/users \
     -H "Content-Type: application/json" \
     -d '{"email": "test@example.com", "password": "password123"}'
   ```

4. **List Users**
   ```bash
   curl http://localhost:3000/api/users?page=1&limit=10
   ```

## 🧪 Testing

### Running Tests
```bash
cargo test
```

### Mock Service Example
```rust
use crate::services::{UserService, MockUserService};

#[tokio::test]
async fn test_user_creation() {
    let mock_service = Arc::new(MockUserService::new());
    // Test with mock service
}
```

## 📝 Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Database Configuration
DATABASE_URL=postgresql://localhost/rust_boilerplate

# Logging
RUST_LOG=debug
```

## 🏛️ Clean Code Principles

### 1. **Single Responsibility**
- Each handler has one clear purpose
- Services handle business logic
- Response helpers handle formatting

### 2. **Dependency Inversion**
- Handlers depend on service traits, not implementations
- Easy to inject mock services for testing

### 3. **Consistent Error Handling**
- All errors follow the same response format
- Detailed validation messages
- Proper HTTP status codes

### 4. **Type Safety**
- Strong typing throughout the application
- Validation at the model level
- Compile-time guarantees

## 🔧 Development

### Adding New Endpoints

1. **Define Request/Response Models**
   ```rust
   #[derive(serde::Deserialize, Validate)]
   pub struct CreateResourceRequest {
       #[validate(length(min = 1))]
       pub name: String,
   }
   ```

2. **Add Service Method**
   ```rust
   #[async_trait]
   pub trait ResourceService {
       async fn create_resource(&self, request: CreateResourceRequest) -> Result<Resource, ServiceError>;
   }
   ```

3. **Implement Handler**
   ```rust
   pub async fn create_resource(
       State(service): State<Arc<dyn ResourceService>>,
       Json(payload): Json<CreateResourceRequest>,
   ) -> Result<Response, Response> {
       // Validation and service call
       match service.create_resource(payload).await {
           Ok(resource) => Ok(success_response(resource).into_response()),
           Err(_) => Err(internal_error_response("Failed to create resource").into_response()),
       }
   }
   ```

4. **Add Route**
   ```rust
   .route("/resources", axum::routing::post(handlers::create_resource))
   ```

## 📊 Response Codes

- `200 OK` - Successful operations
- `201 Created` - Resource created successfully
- `400 Bad Request` - Validation errors or malformed requests
- `401 Unauthorized` - Authentication required
- `403 Forbidden` - Permission denied
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server-side errors

## 🎯 Best Practices Implemented

1. **Consistent Response Format**: All endpoints return standardized JSON
2. **Proper HTTP Methods**: GET, POST, PUT, DELETE used appropriately
3. **Input Validation**: All requests validated before processing
4. **Error Handling**: Comprehensive error responses with details
5. **Logging**: Structured logging for debugging and monitoring
6. **Testing**: Mock services for easy unit testing
7. **Clean Architecture**: Clear separation of concerns
8. **Type Safety**: Leverages Rust's type system
9. **Async/Await**: Non-blocking operations throughout
10. **Configuration**: Environment-based configuration management

## 🚀 Production Ready

This boilerplate includes production-ready features:

- Health check endpoints for container orchestration
- Structured logging for monitoring
- Error tracking with detailed information
- Graceful error handling
- Type-safe configuration
- Comprehensive testing support

## 🤝 Contributing

This boilerplate follows Rust best practices and clean code principles. When contributing:

1. Follow the existing code structure
2. Use the standardized response format
3. Add tests for new features
4. Update documentation
5. Use proper error handling

---

**Built with ❤️ using Rust, Axum, and clean architecture principles**