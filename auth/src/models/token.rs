//! Token-related data structures

use serde::{Deserialize, Serialize};

use crate::models::user::UserSchema;

/// Response containing authentication tokens and related data
#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct TokenResponse {
    /// JWT access token
    pub access_token: String,
    /// Token type (usually "bearer")
    pub token_type: String,
    /// Token expiration time in seconds
    pub expires_in: u64,
    /// Timestamp when token expires
    pub expires_at: u64,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// Associated user data
    pub user: Option<UserSchema>,
    /// OAuth provider token if applicable
    pub provider_token: String,
    /// OAuth provider refresh token if applicable
    pub provider_refresh_token: String,
    /// Details about weak password if detected
    pub weak_password: Option<WeakPasswordError>,
}

/// Error details when a weak password is detected
#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct WeakPasswordError {
    /// Error message describing the weak password
    pub message: String,
    /// List of reasons why the password is considered weak
    pub reasons: Vec<String>,
}
