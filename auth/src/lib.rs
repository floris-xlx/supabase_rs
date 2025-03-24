//! Supabase Auth client library for Rust
//!
//! This crate provides a Rust interface to the Supabase Auth API.
//! It handles authentication operations like signup, signin, token refresh,
//! and user management.

pub use error::AuthError;
pub use models::user::UserSchema as User;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use tracing::{debug, error, instrument, warn};
#[allow(unused)]
pub use ErrorSchema as Error;

mod error;
mod get_user;
pub mod models;
mod refresh_token;
mod signin_with_password;
mod signout;
mod signup;

/// Main client for interacting with Supabase Auth
#[derive(Clone, Debug)]
pub struct AuthClient {
    /// HTTP client for making requests
    http_client: Client,
    /// Base URL for the Supabase API
    supabase_api_url: String,
    /// Anonymous API key for authentication
    supabase_anon_key: String,

    session: RefCell<Option<AuthSession>>,
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
        #[cfg(feature = "rustls")]
        let http_client = Client::builder().use_rustls_tls().build()?;

        #[cfg(not(feature = "rustls"))]
        let http_client = Client::new();

        Self::with_http_client(api_url, anon_key, http_client)
    }

    /// Creates a new AuthClient instance
    ///
    /// # Arguments
    /// * `api_url` - Base URL for the Supabase API
    /// * `anon_key` - Anonymous API key for authentication
    ///
    /// # Returns
    /// * `Result<Self, anyhow::Error>` - New client instance or error
    pub fn with_http_client(
        api_url: &str,
        anon_key: &str,
        http_client: Client,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            http_client,
            supabase_api_url: api_url.to_owned(),
            supabase_anon_key: anon_key.to_owned(),
            session: RefCell::new(None),
        })
    }

    /// Gets the current user details if there is an existing session, or None if not.
    ///
    /// # Returns
    /// * `Option<AuthSession>` - User's session data if authenticated, None if not found
    pub fn session(&self) -> Option<AuthSession> {
        self.session.borrow().as_ref().cloned()
    }

    /// Checks if the client has an active session
    ///
    /// # Returns
    /// * `bool` - True if the client has an active session, false otherwise
    pub fn is_authenticated(&self) -> bool {
        self.session.borrow().is_some()
    }

    /// Handles HTTP response status codes and maps them to appropriate AuthErrors
    ///
    /// # Arguments
    /// * `http_response` - The HTTP response
    ///
    /// # Returns
    /// `Result<Option<T>, AuthError>` - Ok(Some) with deserialized body if status is successful,
    /// Ok(None) when the response is successful but has no body, and appropriate error otherwise
    #[instrument]
    async fn handle_response_code<T>(&self, http_response: Response) -> Result<Option<T>, AuthError>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = http_response.status();
        debug!("response.status = {}", status);
        if status.is_success() {
            if status == StatusCode::NO_CONTENT {
                return Ok(None);
            }

            let resp_text = match http_response.text().await {
                Ok(resp_text) => resp_text,
                Err(e) => {
                    error!("{e:?}");
                    return Err(AuthError::Http);
                }
            };
            let t = match serde_json::from_str::<T>(&resp_text) {
                Ok(token_response) => token_response,
                Err(e) => {
                    error!("{e:?}");
                    return Err(AuthError::Internal);
                }
            };

            Ok(Some(t))
        } else {
            let response_text = match &http_response.text().await {
                Ok(text) => text.clone(),
                Err(e) => e.to_string(),
            };
            debug!("response.text = {response_text}");
            match status {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(AuthError::NotAuthorized),
                StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
                    Err(AuthError::InvalidParameters(response_text.to_string()))
                }
                StatusCode::NOT_FOUND | StatusCode::NOT_ACCEPTABLE => Err(AuthError::NotFound),
                StatusCode::INTERNAL_SERVER_ERROR | _ => Err(AuthError::GeneralError),
            }
        }
    }

    fn apply_http_headers(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let session = self.session.borrow();
        let bearer_token = match session.as_ref() {
            Some(session) => &session.access_token,
            None => &self.supabase_anon_key,
        };

        request
            .header("apiKey", &self.supabase_anon_key)
            .bearer_auth(bearer_token)
    }

    fn http_get(&self, url: &str) -> reqwest::RequestBuilder {
        let request = self
            .http_client
            .get(format!("{}/auth/v1/{}", self.supabase_api_url, url));
        self.apply_http_headers(request)
    }

    fn http_post(&self, url: &str) -> reqwest::RequestBuilder {
        let request = self
            .http_client
            .post(format!("{}/auth/v1/{}", self.supabase_api_url, url));
        self.apply_http_headers(request)
    }
}

/// Represents an authenticated session with Supabase
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
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
    use crate::AuthClient;
    use anyhow::Result;

    // Dummy email for testing
    // A Supabase user with this email must exist
    pub(crate) const TEST_USER_EMAIL: &'static str = "dummy@supabase.rs";
    pub(crate) const TEST_USER_PASSWD: &'static str = "supabase";

    pub(crate) async fn get_auth_client() -> Result<AuthClient> {
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::try_init().ok();

        let supabase_url = std::env::var("SUPABASE_URL")?;
        let supabase_key = std::env::var("SUPABASE_KEY")?;

        AuthClient::new(&supabase_url, &supabase_key)
    }
}
