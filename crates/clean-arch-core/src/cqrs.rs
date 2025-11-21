//! CQRS (Command Query Responsibility Segregation) pattern implementation.

use async_trait::async_trait;
use std::fmt::Debug;

/// Marker trait for commands.
///
/// Commands represent an intent to change the system state.
/// They should be named as imperative verbs (e.g., `CreateUser`, `UpdateOrder`).
pub trait Command: Debug + Send + Sync + 'static {
    /// The type returned when the command is successfully handled.
    type Output: Send;
}

/// Marker trait for queries.
///
/// Queries represent a request for data without side effects.
/// They should be named as questions (e.g., `GetUserById`, `ListOrders`).
pub trait Query: Debug + Send + Sync + 'static {}

/// Handler for commands.
///
/// Command handlers contain the business logic for processing commands.
/// They typically interact with repositories to persist changes.
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// The error type that can be returned by this handler.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Handles the command and returns the result.
    async fn handle(&self, command: C) -> Result<C::Output, Self::Error>;
}

/// Handler for queries.
///
/// Query handlers retrieve data without causing side effects.
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// The type returned by this query handler.
    type Output: Send;

    /// The error type that can be returned by this handler.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Handles the query and returns the result.
    async fn handle(&self, query: Q) -> Result<Self::Output, Self::Error>;
}
