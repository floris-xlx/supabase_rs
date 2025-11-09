//! # Error Handling and Types
//!
//! This module provides comprehensive error handling utilities for the Supabase client.
//! It defines structured error types, recovery strategies, and debugging utilities
//! to help developers handle failures gracefully.
//!
//! ## 🎯 Error Philosophy
//!
//! The SDK uses a layered error handling approach:
//! - **Structured Errors**: Type-safe error variants for different failure modes
//! - **String Errors**: Simple error messages for end-user operations
//! - **Recovery Guidance**: Clear error messages with actionable solutions
//! - **Debug Support**: Detailed error context for troubleshooting
//!
//! ## 🏗️ Error Types
//!
//! | Error Type | Trigger | Recovery Strategy |
//! |------------|---------|-------------------|
//! | `UnknownError` | Unexpected failures | Check logs, retry operation |
//! | `ApiKeyMissing` | Missing credentials | Set SUPABASE_KEY environment variable |
//! | `AuthorizationFailed` | Invalid/expired key | Verify API key and permissions |
//! | `InvalidQuery` | Malformed requests | Check query syntax and parameters |
//! | `ReqwestError` | Network/HTTP issues | Check connectivity, retry with backoff |
//! | `EnvironmentError` | Missing env vars | Set required environment variables |
//!
//! ## 📖 Usage Examples
//!
//! ### Basic Error Handling
//!
//! ```rust,no_run
//! use supabase_rs::{SupabaseClient, errors::ErrorTypes};
//!
//! # async fn example() -> Result<(), ErrorTypes> {
//! let client = match SupabaseClient::new(
//!     std::env::var("SUPABASE_URL").map_err(ErrorTypes::EnvironmentError)?,
//!     std::env::var("SUPABASE_KEY").map_err(ErrorTypes::EnvironmentError)?,
//! ) {
//!     Ok(client) => client,
//!     Err(ErrorTypes::ReqwestError(e)) => {
//!         eprintln!("Failed to create HTTP client: {}", e);
//!         return Err(ErrorTypes::UnknownError);
//!     },
//!     Err(e) => return Err(e),
//! };
//! # Ok(())
//! # }
//! ```
//!
//! ### Error Recovery Patterns
//!
//! ```rust,no_run
//! use supabase_rs::SupabaseClient;
//! use serde_json::json;
//! use tokio::time::{sleep, Duration};
//!
//! # async fn example() -> Result<(), String> {
//! # let client = SupabaseClient::new("url".to_string(), "key".to_string()).unwrap();
//! // Retry logic for transient failures
//! async fn insert_with_retry(
//!     client: &SupabaseClient,
//!     table: &str,
//!     data: serde_json::Value,
//!     max_retries: u32
//! ) -> Result<String, String> {
//!     for attempt in 1..=max_retries {
//!         match client.insert(table, data.clone()).await {
//!             Ok(id) => return Ok(id),
//!             Err(err) if err.contains("timeout") && attempt < max_retries => {
//!                 println!("Attempt {} failed with timeout, retrying...", attempt);
//!                 sleep(Duration::from_millis(1000 * attempt as u64)).await;
//!                 continue;
//!             },
//!             Err(err) => return Err(err),
//!         }
//!     }
//!     Err("Max retries exceeded".to_string())
//! }
//! # Ok(())
//! # }
//! ```

use anyhow::Error;

/// Comprehensive error types for Supabase operations.
///
/// This enum provides structured error handling with specific variants for different
/// failure scenarios. Each variant includes descriptive error messages and, where
/// applicable, wraps underlying error types for detailed debugging.
///
/// # Error Categories
///
/// - **Authentication**: `ApiKeyMissing`, `AuthorizationFailed`
/// - **Request**: `InvalidQuery`, `ReqwestError`
/// - **Environment**: `EnvironmentError`
/// - **Generic**: `UnknownError`
///
/// # Examples
///
/// ```rust,no_run
/// use supabase_rs::errors::ErrorTypes;
///
/// fn handle_error(error: ErrorTypes) {
///     match error {
///         ErrorTypes::ApiKeyMissing => {
///             eprintln!("🔑 Set your SUPABASE_KEY environment variable");
///         },
///         ErrorTypes::AuthorizationFailed => {
///             eprintln!("🚫 Check your API key permissions");
///         },
///         ErrorTypes::ReqwestError(e) => {
///             eprintln!("🌐 Network error: {}", e);
///         },
///         ErrorTypes::EnvironmentError(e) => {
///             eprintln!("⚙️ Environment setup error: {}", e);
///         },
///         _ => eprintln!("❌ Unexpected error occurred"),
///     }
/// }
/// ```
#[derive(thiserror::Error, Debug)]
pub enum ErrorTypes {
    /// Catch-all for unexpected errors that don't fit other categories
    #[error("Unknown error occurred")]
    UnknownError,

    /// API key was not provided or is empty
    #[error("API key is missing - set SUPABASE_KEY environment variable")]
    ApiKeyMissing,

    /// API key is invalid, expired, or lacks required permissions
    #[error("Authorization failed - verify API key and permissions")]
    AuthorizationFailed,

    /// Query syntax is invalid or contains unsupported operations
    #[error("Invalid query - check syntax and parameters")]
    InvalidQuery,

    /// HTTP client or network-related errors
    #[error("Network error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Environment variable is missing or invalid
    #[error("Environment configuration error: {0}")]
    EnvironmentError(#[from] std::env::VarError),
}

/// Type alias for Results using [`ErrorTypes`].
///
/// This provides a convenient shorthand for functions that return structured errors:
/// ```rust,no_run
/// use supabase_rs::errors::Result;
///
/// fn example_operation() -> Result<String> {
///     // Function that might fail with ErrorTypes
///     Ok("success".to_string())
/// }
/// ```
pub type Result<Type> = std::result::Result<Type, ErrorTypes>;

/// Creates an unknown error result.
///
/// This function is used internally when an unexpected error occurs that doesn't
/// fit into the other error categories. It returns an `anyhow::Error` for maximum
/// compatibility with error handling chains.
///
/// # Returns
/// Always returns `Err(anyhow::Error)` with a generic unknown error message.
///
/// # Examples
/// ```rust,no_run
/// use supabase_rs::errors::unknown_error;
///
/// # async fn example() {
/// let result = unknown_error().await;
/// assert!(result.is_err());
/// # }
/// ```
pub async fn unknown_error() -> std::result::Result<(), Error> {
    Err(Error::msg("SUPABASE_RS: Unknown error occurred"))
}

/// Creates an API key missing error result.
///
/// Used when operations fail due to missing or empty API key configuration.
/// Provides actionable guidance for resolving the authentication issue.
///
/// # Returns
/// Always returns `Err(anyhow::Error)` with API key missing guidance.
pub async fn api_key_missing_error() -> std::result::Result<(), Error> {
    Err(Error::msg(
        "SUPABASE_RS: API key is missing - set SUPABASE_KEY environment variable",
    ))
}

/// Creates an authorization failed error result.
///
/// Used when API requests fail due to invalid, expired, or insufficient API key permissions.
/// Indicates that the key exists but lacks the required access level.
///
/// # Returns
/// Always returns `Err(anyhow::Error)` with authorization failure guidance.
pub async fn authorization_failed_error() -> std::result::Result<(), Error> {
    Err(Error::msg(
        "SUPABASE_RS: Authorization failed - verify API key permissions and expiration",
    ))
}

/// Creates an invalid query error result.
///
/// Used when query construction or execution fails due to malformed syntax,
/// invalid parameters, or unsupported operations.
///
/// # Returns
/// Always returns `Err(anyhow::Error)` with query validation guidance.
pub async fn invalid_query_error() -> std::result::Result<(), Error> {
    Err(Error::msg(
        "SUPABASE_RS: Invalid query - check syntax, parameters, and table/column names",
    ))
}
