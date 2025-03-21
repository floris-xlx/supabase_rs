//! Error types for the Supabase Auth client

use serde_json::Error;
use thiserror::Error;

/// Possible errors that can occur during authentication operations
#[derive(Debug, Default, Clone, Error)]
pub enum AuthError {
    /// User is not authorized to perform the requested operation
    #[error("not authorized")]
    NotAuthorized,

    /// Invalid parameters were provided
    #[error("invalid parameters")]
    InvalidParameters,

    /// HTTP request failed
    #[error("http error")]
    Http,

    /// JSON serialization/deserialization error
    #[error("http error")]
    Json(String),

    /// Internal library error occurred
    #[error("internal library error")]
    Internal,

    /// Requested resource was not found
    #[error("resource not found")]
    NotFound,

    /// General GoTrue API error
    #[error("general gotrue error")]
    #[default]
    GeneralError,
}

impl From<serde_json::Error> for AuthError {
    fn from(err: Error) -> Self {
        AuthError::Json(err.to_string())
    }
}
