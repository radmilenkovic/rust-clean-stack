# rust-clean-stack

Rust toolkit for building production-ready API services using Clean Architecture principles.

## Overview

This project provides reusable Rust crates for implementing Clean Architecture patterns:

| Crate                                      | Description                                      | Status       |
| ------------------------------------------ | ------------------------------------------------ | ------------ |
| [clean-arch-core](./crates/clean-arch-core)   | CQRS traits, Repository patterns, Error handling | ✅ Published |
| [clean-dto](./crates/clean-dto)               | Derive macros for DTO - Entity conversion        | 🚧 WIP       |
| [clean-dto-macros](./crates/clean-dto-macros) | Proc-macro implementation                        | 🚧 WIP       |
| [tracing-serilog](./crates/tracing-serilog)   | Serilog-compatible JSON formatter                | 🚧 WIP       |
| [axum-scheduler](./crates/axum-scheduler)     | Background job scheduler for Axum                | 🚧 WIP       |

## Architecture

The project follows Clean Architecture with four layers:

- **API Layer** - REST (Axum) or gRPC (Tonic) endpoints
- **Application Layer** - Commands, Queries, Handlers, DTOs
- **Domain Layer** - Entities, Value Objects, Repository traits (pure Rust, no dependencies)
- **Infrastructure Layer** - Database implementations, Auth, External services

**Dependency Rule:** Dependencies point inward. Domain has no external dependencies. Infrastructure implements Domain traits.

## Installation

```toml
[dependencies]
clean-arch-core = "0.1"
```

## Quick Start

### Define a Command

```rust
use clean_arch_core::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}

impl Command for CreateUser {
    type Output = Uuid;
}
```

### Implement a Handler

```rust
pub struct CreateUserHandler<R: UserRepository> {
    repo: R,
}

#[async_trait]
impl<R: UserRepository> CommandHandler<CreateUser> for CreateUserHandler<R> {
    type Error = AppError;

    async fn handle(&self, cmd: CreateUser) -> Result<Uuid, Self::Error> {
        if cmd.email.is_empty() {
            return Err(AppError::validation("email", "Email is required"));
        }

        let user_id = Uuid::new_v4();
        Ok(user_id)
    }
}
```

### Define a Query

```rust
#[derive(Debug)]
pub struct GetUserById {
    pub id: Uuid,
}

impl Query for GetUserById {}
```

### Create Typed Entity IDs

```rust
use clean_arch_core::entity_id;

entity_id!(UserId);
entity_id!(OrderId);
```

## Error Handling

```rust
use clean_arch_core::error::AppError;

// Not found (404)
let err = AppError::not_found("User", "123");

// Validation error (400)
let err = AppError::validation("email", "Invalid format");

// Business rule violation (422)
let err = AppError::business_rule("Insufficient funds");

// Get HTTP status
assert_eq!(err.http_status_code(), 422);
```

## Features

- CQRS Pattern - Separate read and write operations
- Repository Pattern - Abstract data persistence
- Typed Entity IDs - Prevent ID mixups at compile time
- Structured Errors - HTTP-aware error handling
- Pagination - Built-in pagination support
- Async/Await - Fully async with Tokio

## Requirements

- Rust 1.85+ (Edition 2024)

## Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo publish -p clean-arch-core
```

## License

MIT OR Apache-2.0
