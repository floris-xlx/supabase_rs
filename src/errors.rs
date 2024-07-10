//! ## Error handling
//! 
//! This module provides error handling utilities for the Supabase client.

use anyhow::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorSupabase {
    pub message: String,
}

impl fmt::Display for ErrorSupabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ErrorSupabase {}

#[derive(Debug)]
pub enum ErrorTypes {
    UnknownError,
    ApiKeyMissing,
    AuthorizationFailed,
    InvalidQuery,
}

impl fmt::Display for ErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorTypes::UnknownError => write!(f, "Unknown error"),
            ErrorTypes::ApiKeyMissing => write!(f, "API key is missing"),
            ErrorTypes::AuthorizationFailed => write!(f, "Authorization failed"),
            ErrorTypes::InvalidQuery => write!(f, "Invalid query"),
        }
    }
}

impl std::error::Error for ErrorTypes {}

pub async fn unknown_error() -> Result<(), Error> {
    Err(Error::msg("\x1b[32mSUPABASE_RS\x1b[0m: \x1b[31munknown error\x1b[0m"))
}

pub async fn api_key_missing_error() -> Result<(), Error> {
    Err(Error::msg("\x1b[32mSUPABASE_RS\x1b[0m: \x1b[31mAPI key is missing\x1b[0m"))
}

pub async fn authorization_failed_error() -> Result<(), Error> {
    Err(Error::msg("\x1b[32mSUPABASE_RS\x1b[0m: \x1b[31mAuthorization failed\x1b[0m"))
}

pub async fn invalid_query_error() -> Result<(), Error> {
    Err(Error::msg("\x1b[32mSUPABASE_RS\x1b[0m: \x1b[31mInvalid query\x1b[0m"))
}