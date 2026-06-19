# Backend Axum Template

A production-grade Rust backend API template built with [Axum](https://github.com/tokio-rs/axum), featuring a clean three-layer architecture with dependency injection, async PostgreSQL via Diesel, and structured JSON logging.

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
[ Repository ]      src/internal/storage/    — Database access (Diesel)
     |
[ PostgreSQL ]
```

Dependency injection is wired explicitly in `src/initiator.rs`: Storage → Modules → Handlers, with each layer receiving its dependencies via `Arc<dyn Trait>`.

## Features

- **Axum 0.8** — async routing, state extraction, JSON handling, middleware
- **Tokio** — multi-threaded async runtime
- **PostgreSQL + Diesel 2.2** — async queries via `diesel-async` with BB8 connection pooling
- **Three-layer architecture** — Storage (repositories), Modules (services), Handlers (controllers)
- **Repository & Service patterns** — trait-based abstractions, mockable for testing
- **Structured JSON logging** — via `tracing` + `tracing-subscriber` (env-filter)
- **Environment-based config** — via `envy` + `dotenvy` (`.env` file support)
- **Typed error handling** — `AppError` enum with automatic HTTP status code mapping via `IntoResponse`
- **Diesel migrations** — with `Makefile` convenience targets

## Prerequisites

- Rust (edition 2024 compatible)
- PostgreSQL 16 (or Docker)
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

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
| `RUST_LOG` | `info,backend_axum_template=debug` | Tracing/logging level filter |

## API Endpoints

| Method | Path | Description | Auth |
|---|---|---|---|
| `POST` | `/api/v1/users` | Create a new user | No |
| `GET` | `/api/v1/users/{id}` | Get user by UUID | `x-api-token` header required |

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
