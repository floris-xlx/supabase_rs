//! ## Error handling
//!
//! This module provides error handling utilities for the Supabase client.

use anyhow::Error;

#[derive(thiserror::Error, Debug)]
pub enum ErrorTypes {
    #[error("Unknown error")]
    UnknownError,
    #[error("API key is missing")]
    ApiKeyMissing,
    #[error("Authorization failed")]
    AuthorizationFailed,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Invalid query")]
    InvalidQuery,
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Environment variable error: {0}")]
    EnvironmentError(#[from] std::env::VarError),
}

pub type Result<Type> = std::result::Result<Type, ErrorTypes>;

pub async fn unknown_error() -> std::result::Result<(), Error> {
    Err(Error::msg("SUPABASE_RS: unknown error"))
}

pub async fn api_key_missing_error() -> std::result::Result<(), Error> {
    Err(Error::msg("SUPABASE_RS: API key is missing"))
}

pub async fn authorization_failed_error() -> std::result::Result<(), Error> {
    Err(Error::msg("SUPABASE_RS: Authorization failed"))
}

pub async fn invalid_query_error() -> std::result::Result<(), Error> {
    Err(Error::msg("SUPABASE_RS: Invalid query"))
}
