//! Repository pattern abstractions for data persistence.

use crate::entity::{Entity, EntityId};
use crate::error::AppError;
use async_trait::async_trait;
use std::fmt::Debug;

/// Result type for repository operations.
pub type RepositoryResult<T> = Result<T, AppError>;

/// Base trait for repositories.
///
/// A repository mediates between the domain and data mapping layers,
/// acting like an in-memory domain object collection.
#[async_trait]
pub trait Repository<E: Entity>: Send + Sync + Debug {
    /// Finds an entity by its unique identifier.
    async fn find_by_id(&self, id: E::Id) -> RepositoryResult<Option<E>>;

    /// Finds all entities matching the given IDs.
    async fn find_by_ids(&self, ids: &[E::Id]) -> RepositoryResult<Vec<E>>;

    /// Returns all entities in the repository.
    async fn find_all(&self) -> RepositoryResult<Vec<E>>;

    /// Checks if an entity with the given ID exists.
    async fn exists(&self, id: E::Id) -> RepositoryResult<bool> {
        Ok(self.find_by_id(id).await?.is_some())
    }

    /// Returns the total count of entities.
    async fn count(&self) -> RepositoryResult<u64>;

    /// Saves (inserts or updates) an entity.
    async fn save(&self, entity: &E) -> RepositoryResult<()>;

    /// Deletes an entity by its ID.
    async fn delete(&self, id: E::Id) -> RepositoryResult<()>;
}

/// Pagination parameters for repository queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pagination {
    /// The page number (0-indexed).
    pub page: u64,
    /// The number of items per page.
    pub per_page: u64,
}

impl Pagination {
    /// Creates a new pagination with the given parameters.
    #[must_use]
    pub const fn new(page: u64, per_page: u64) -> Self {
        Self { page, per_page }
    }

    /// Returns the offset for SQL queries.
    #[must_use]
    pub const fn offset(&self) -> u64 {
        self.page * self.per_page
    }

    /// Returns the limit for SQL queries.
    #[must_use]
    pub const fn limit(&self) -> u64 {
        self.per_page
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 20,
        }
    }
}

/// A paginated result containing items and metadata.
#[derive(Debug, Clone)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// The total number of items across all pages.
    pub total_items: u64,
    /// The current page number (0-indexed).
    pub page: u64,
    /// The number of items per page.
    pub per_page: u64,
}

impl<T> Page<T> {
    /// Creates a new page with the given items and metadata.
    #[must_use]
    pub fn new(items: Vec<T>, total_items: u64, pagination: Pagination) -> Self {
        Self {
            items,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    /// Returns the total number of pages.
    #[must_use]
    pub fn total_pages(&self) -> u64 {
        if self.per_page == 0 {
            return 0;
        }
        (self.total_items + self.per_page - 1) / self.per_page
    }

    /// Returns whether there is a next page.
    #[must_use]
    pub fn has_next(&self) -> bool {
        self.page + 1 < self.total_pages()
    }

    /// Returns whether there is a previous page.
    #[must_use]
    pub fn has_previous(&self) -> bool {
        self.page > 0
    }
}
