//! Entity and Aggregate Root abstractions for domain modeling.

use std::fmt::{Debug, Display};
use std::hash::Hash;
use uuid::Uuid;

/// Trait for entity identifiers.
///
/// Entity IDs should be unique within their entity type and immutable.
pub trait EntityId: Debug + Display + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + 'static {
    /// Creates a new unique identifier.
    fn new() -> Self;

    /// Returns the underlying UUID representation.
    fn as_uuid(&self) -> Uuid;

    /// Creates an ID from an existing UUID.
    fn from_uuid(uuid: Uuid) -> Self;
}

/// Base trait for domain entities.
///
/// Entities have identity that persists through time and across different representations.
/// Two entities are equal if they have the same ID, regardless of their other attributes.
pub trait Entity: Debug + Send + Sync {
    /// The type of this entity's identifier.
    type Id: EntityId;

    /// Returns the entity's unique identifier.
    fn id(&self) -> Self::Id;
}

/// Mixin for entities with timestamp tracking.
pub trait Timestamped {
    /// Returns when the entity was created.
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;

    /// Returns when the entity was last updated.
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc>;
}

/// Mixin for soft-deletable entities.
pub trait SoftDeletable {
    /// Returns whether the entity has been soft-deleted.
    fn is_deleted(&self) -> bool;

    /// Returns when the entity was deleted, if applicable.
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>>;
}

/// Macro to generate a typed entity ID.
///
/// # Example
///
/// ```rust
/// use clean_arch_core::entity_id;
///
/// entity_id!(UserId);
/// entity_id!(OrderId);
/// ```
#[macro_export]
macro_rules! entity_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(uuid::Uuid);

        impl $crate::entity::EntityId for $name {
            fn new() -> Self {
                Self(uuid::Uuid::now_v7())
            }

            fn as_uuid(&self) -> uuid::Uuid {
                self.0
            }

            fn from_uuid(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(uuid: uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl From<$name> for uuid::Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
