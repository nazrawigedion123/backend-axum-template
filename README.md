# Backend Axum Template

A production-grade Rust backend API template built with [Axum](https://github.com/tokio-rs/axum), featuring a clean three-layer architecture with dependency injection, async PostgreSQL via SQLx, and structured JSON logging.

## Architecture

```
HTTP Request
     |
[ Middleware ]
     |
[ Route Handler ]   src/internal/handler/    — HTTP concerns (request parsing, response writing)
     |
[ Service Layer ]   src/internal/module/     — Business logic & validation
     |
[ Repository ]      src/internal/storage/    — Database access (SQLx)
     |
[ PostgreSQL ]
```

Dependency injection is wired explicitly in `src/initiator.rs`: Storage → Modules → Handlers, with each layer receiving its dependencies via `Arc<dyn Trait>`.

## Project Structure

```
src/
├── main.rs                 # Entry point — initializes logger, builds app, starts server
├── config.rs               # AppConfig — loads env vars into a typed struct (envy + dotenvy)
├── docs.rs                 # OpenAPI 3.0 spec generation via utoipa
├── initiator.rs            # Composition root — wires DI: repos → services → handlers
├── platform/               # Infrastructure concerns
│   ├── mod.rs
│   └── logger.rs           # tracing-subscriber JSON logger setup
└── internal/
    ├── mod.rs
    ├── constant/           # Shared domain types
    │   ├── mod.rs
    │   ├── dto.rs          # Request/response DTOs
    │   ├── errors.rs       # AppError enum + IntoResponse impl for HTTP mapping
    │   └── model/
    │       ├── mod.rs
    │       └── user.rs     # Domain models (UserModel, NewUserModel)
    ├── handler/            # HTTP layer — request parsing, response writing
    │   ├── mod.rs
    │   ├── user_handler/
    │   │   └── mod.rs      # UserHandler — route functions (create, get)
    │   └── middleware/
    │       ├── mod.rs
    │       └── sample.rs   # Dummy auth middleware (x-api-token check)
    ├── module/             # Business logic layer — validation, orchestration
    │   ├── mod.rs
    │   └── user_service/
    │       └── mod.rs      # DefaultUserService — create/get user logic
    ├── routes/             # Route registration
    │   ├── mod.rs
    │   └── user_routes/
    │       └── mod.rs      # User endpoint route registration
    └── storage/            # Database access layer — SQLx queries
        ├── mod.rs
        └── user_storage/
            └── mod.rs      # PostgresUserRepository — SQLx query impls
```

## Features

- **Axum 0.8** — async routing, state extraction, JSON handling, middleware
- **Tokio** — multi-threaded async runtime
- **PostgreSQL + SQLx 0.8** — async queries with compile-time checked SQL and connection pooling
- **Three-layer architecture** — Storage (repositories), Modules (services), Handlers (controllers)
- **Repository & Service patterns** — trait-based abstractions, mockable for testing
- **Structured JSON logging** — via `tracing` + `tracing-subscriber` (env-filter)
- **Environment-based config** — via `envy` + `dotenvy` (`.env` file support)
- **Typed error handling** — `AppError` enum with automatic HTTP status code mapping via `IntoResponse`
- **SQLx migrations** — with `Makefile` convenience targets

## Prerequisites

- Rust (edition 2024 compatible)
- PostgreSQL 16 (or Docker)
- SQLx CLI: `cargo install sqlx-cli`

## Getting Started

```bash
# 1. Start PostgreSQL
docker compose up -d

# 2. Copy environment configuration
cp .env.example .env

# 3. Set up the database and run migrations
make db-setup

# 4. Start the server
cargo run
```

The server starts on `http://127.0.0.1:8080`.

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `SERVER_PORT` | `8080` | Port to bind the HTTP server |
| `DATABASE_URL` | (required) | PostgreSQL connection string |
| `ENV` | `production` | Application environment |
| `RUST_LOG` | `info,backend_axum_template=debug` | Tracing/logging level filter |

## API Endpoints

| Method | Path | Description | Auth |
|---|---|---|---|
| `POST` | `/api/v1/users` | Create a new user | No |
| `GET` | `/api/v1/users/{id}` | Get user by UUID | `x-api-token` header required |
| `GET` | `/api/v1/users/get-by-username/{username}` | Get user by username | No |

### Create User

```json
POST /api/v1/users
{
  "username": "johndoe",
  "email": "john@example.com"
}
```

### Get User

```http
GET /api/v1/users/550e8400-e29b-41d4-a716-446655440000
x-api-token: any-non-empty-value
```

## Database Migrations

```bash
make db-status          # List migration status
make migration-create name=<name>  # Generate new migration
make migration-up       # Run pending migrations
make migration-down     # Revert the most recent migration
make migration-redo     # Re-run the latest migration
make db-setup           # Create database + run all migrations
```

## Build

```bash
cargo build
cargo build --release
```
