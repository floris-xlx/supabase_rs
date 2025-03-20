//! Supabase Auth client library for Rust
//!
//! This crate provides a Rust interface to the Supabase Auth API.
//! It handles authentication operations like signup, signin, token refresh,
//! and user management.

use std::fmt::{Debug, Display, Formatter};

use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use error::AuthError;
pub use models::user::UserSchema as User;
#[allow(unused)]
pub use ErrorSchema as Error;

mod delete_user;
mod error;
mod get_user;
mod logout;
pub mod models;
mod refresh_token;
mod signin_with_password;
mod signup;
mod util;

#[derive(Clone)]
struct PostgrestNewtype(Postgrest);

impl Debug for PostgrestNewtype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Postgrest").finish()
    }
}
/// Main client for interacting with Supabase Auth
#[derive(Clone, Debug)]
pub struct AuthClient {
    /// HTTP client for making requests
    http_client: reqwest::Client,
    /// Base URL for the Supabase API
    supabase_api_url: String,
    /// Anonymous API key for authentication
    supabase_anon_key: String,
    /// Client for making PostgreSQL REST API calls
    #[allow(dead_code)]
    postgrest_client: PostgrestNewtype,

    session: Option<AuthSession>,
}

impl AuthClient {
    /// Creates a new AuthClient instance
    ///
    /// # Arguments
    /// * `api_url` - Base URL for the Supabase API
    /// * `anon_key` - Anonymous API key for authentication
    ///
    /// # Returns
    /// * `Result<Self, anyhow::Error>` - New client instance or error
    pub fn new(api_url: &str, anon_key: &str) -> anyhow::Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            supabase_api_url: api_url.to_owned(),
            supabase_anon_key: anon_key.to_owned(),
            postgrest_client: PostgrestNewtype(
                Postgrest::new(format!("{}/rest/v1/", api_url.to_owned()))
                    .schema("auth")
                    .insert_header("apikey", anon_key.to_owned()),
            ),
            session: None,
        })
    }

    #[allow(dead_code)]
    fn postgrest(&self) -> &Postgrest {
        &self.postgrest_client.0
    }
}

/// Represents an authenticated session with Supabase
#[derive(Debug, Clone)]
pub struct AuthSession {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
    pub user: Option<User>,
}

/// Represents an error response from the Supabase Auth API
#[derive(Debug, Error, Deserialize, Serialize)]
pub struct ErrorSchema {
    /// Numeric error code
    pub code: Option<u8>,
    /// Error type/name
    pub error: Option<String>,
    /// Detailed error description
    pub error_description: Option<String>,
    /// Error message
    pub msg: Option<String>,
}

impl Display for ErrorSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref e) = self.error {
            f.write_str(e)?;
            return Ok(());
        }
        if let Some(ref msg) = self.msg {
            f.write_str(msg)?;
            return Ok(());
        }
        Err(std::fmt::Error)
    }
}

/// Types of user identifiers supported for authentication
#[derive(Debug)]
pub enum IdType {
    /// Email address
    Email(String),
    /// Phone number
    PhoneNumber(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    // Dummy email for testing
    // A Supabase user with this email must exist
    pub(crate) const TEST_USER_EMAIL: &'static str = "dummy@supabase.rs";
    pub(crate) const TEST_USER_PASSWD: &'static str = "supabase";

    pub(crate) async fn get_auth_client() -> Result<AuthClient> {
        dotenv::dotenv().ok();

        let supabase_url = std::env::var("SUPABASE_URL")?;
        let supabase_key = std::env::var("SUPABASE_KEY")?;

        AuthClient::new(&supabase_url, &supabase_key)
    }
}
