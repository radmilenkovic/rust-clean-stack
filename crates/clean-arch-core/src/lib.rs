//! # clean-arch-core
//!
//! Core traits and abstractions for building applications with Clean Architecture principles.

pub mod cqrs;
pub mod entity;
pub mod error;
pub mod repository;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::cqrs::{Command, CommandHandler, Query, QueryHandler};
    pub use crate::entity::{Entity, EntityId};
    pub use crate::error::{AppError, AppResult, ErrorKind};
    pub use crate::repository::{Repository, RepositoryResult};
    pub use async_trait::async_trait;
}

// Re-export commonly used items at crate root
pub use cqrs::*;
pub use entity::*;
pub use error::*;
pub use repository::*;
