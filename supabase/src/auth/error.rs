//! Error types for the Supabase Auth client

use kinded::Kinded;
use thiserror::Error;

/// Possible errors that can occur during authentication operations
#[derive(Debug, Default, Clone, Copy, Error, Kinded)]
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
