# clean-arch-core

Core traits and abstractions for building applications with Clean Architecture principles in Rust.

## Features

- **CQRS Pattern** - `Command`, `Query`, `CommandHandler`, `QueryHandler` traits
- **Repository Pattern** - Generic `Repository` trait with pagination support
- **Entity Abstractions** - `Entity`, `EntityId`, `Timestamped` traits
- **Error Handling** - Standardized `AppError` and `AppResult` types

## Installation

```toml
[dependencies]
clean-arch-core = "0.1"
```

## Quick Example

```rust
use clean_arch_core::prelude::*;

// Define a command
#[derive(Debug)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
}

impl Command for CreateUser {
    type Output = Uuid;
}

// Implement handler
pub struct CreateUserHandler;

#[async_trait]
impl CommandHandler<CreateUser> for CreateUserHandler {
    type Error = AppError;

    async fn handle(&self, cmd: CreateUser) -> Result<Uuid, Self::Error> {
        // your logic here
        todo!()
    }
}
```

## License

MIT OR Apache-2.0
