//! Standardized error types for Clean Architecture applications.

use std::fmt;
use thiserror::Error;

/// Result type alias using `AppError`.
pub type AppResult<T> = Result<T, AppError>;

/// The kind/category of an application error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The requested resource was not found.
    NotFound,
    /// The request was invalid or malformed.
    Validation,
    /// The operation conflicts with the current state.
    Conflict,
    /// The user is not authenticated.
    Unauthorized,
    /// The user is not authorized to perform this operation.
    Forbidden,
    /// A business rule was violated.
    BusinessRule,
    /// An external service failed.
    ExternalService,
    /// A database operation failed.
    Database,
    /// An internal/unexpected error occurred.
    Internal,
}

impl ErrorKind {
    /// Returns the HTTP status code typically associated with this error kind.
    #[must_use]
    pub const fn http_status_code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Validation => 400,
            Self::Conflict => 409,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::BusinessRule => 422,
            Self::ExternalService => 502,
            Self::Database => 500,
            Self::Internal => 500,
        }
    }

    /// Returns a human-readable name for this error kind.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotFound => "NOT_FOUND",
            Self::Validation => "VALIDATION_ERROR",
            Self::Conflict => "CONFLICT",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::BusinessRule => "BUSINESS_RULE_VIOLATION",
            Self::ExternalService => "EXTERNAL_SERVICE_ERROR",
            Self::Database => "DATABASE_ERROR",
            Self::Internal => "INTERNAL_ERROR",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Main application error type.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct AppError {
    kind: ErrorKind,
    message: String,
    code: Option<String>,
    field: Option<String>,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl AppError {
    /// Creates a new `AppError` with the given kind and message.
    #[must_use]
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            code: None,
            field: None,
            source: None,
        }
    }

    /// Creates a new "not found" error.
    #[must_use]
    pub fn not_found(entity: &str, id: impl fmt::Display) -> Self {
        Self::new(ErrorKind::NotFound, format!("{entity} with ID '{id}' not found"))
    }

    /// Creates a new validation error.
    #[must_use]
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        let field_name = field.into();
        Self {
            kind: ErrorKind::Validation,
            message: message.into(),
            code: None,
            field: Some(field_name),
            source: None,
        }
    }

    /// Creates a new conflict error.
    #[must_use]
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Conflict, message)
    }

    /// Creates a new unauthorized error.
    #[must_use]
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Unauthorized, message)
    }

    /// Creates a new forbidden error.
    #[must_use]
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Forbidden, message)
    }

    /// Creates a new business rule violation error.
    #[must_use]
    pub fn business_rule(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::BusinessRule, message)
    }

    /// Creates a new database error.
    #[must_use]
    pub fn database(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Database, message)
    }

    /// Creates a new internal error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal, message)
    }

    /// Returns the error kind.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the field name for validation errors.
    #[must_use]
    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    /// Returns the HTTP status code for this error.
    #[must_use]
    pub const fn http_status_code(&self) -> u16 {
        self.kind.http_status_code()
    }
}
