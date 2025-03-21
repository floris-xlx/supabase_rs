//! Handles user signup operations

use std::collections::HashMap;

use log::error;
use serde::{Deserialize, Serialize};
use tracing::{trace_span, Instrument};

use crate::error::AuthError;
use crate::models::user::UserSchema;
use crate::util::handle_response_code;
use crate::{AuthClient, AuthSession, IdType};

/// Request payload for user signup
#[derive(Debug, Serialize, Deserialize)]
struct SignupRequest {
    /// User's email address
    pub email: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User's password
    pub password: String,
    /// Additional user metadata
    pub data: Option<HashMap<String, String>>,
}

/// Response from a successful signup
#[derive(Debug, Serialize, Deserialize)]
struct SignupResponse {
    /// JWT access token
    pub access_token: String,
    /// Token type (usually "bearer")
    pub token_type: String,
    /// Token expiration time in seconds
    pub expires_in: i64,
    /// Timestamp when token expires
    pub expires_at: i64,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// Created user data
    pub user: UserSchema,
}

impl AuthClient {
    pub async fn signup(
        &self,
        signup_id_type: IdType,
        password: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<AuthSession, AuthError> {
        let body = match signup_id_type {
            IdType::Email(email) => SignupRequest {
                email: Some(email),
                phone_number: None,
                password: password.to_string(),
                data: metadata,
            },
            IdType::PhoneNumber(phone_number) => SignupRequest {
                email: None,
                phone_number: Some(phone_number),
                password: password.to_string(),
                data: metadata,
            },
        };

        let resp = match self
            .http_client
            .post(format!("{}/auth/v1/signup", self.supabase_api_url))
            .header("apiKey", &self.supabase_anon_key)
            .bearer_auth(&self.supabase_anon_key)
            .json(&body)
            .send()
            .instrument(trace_span!("gotrue create user"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("{e:?}");
                return Err(AuthError::Http);
            }
        };

        let session: AuthSession = handle_response_code(resp).await?;

        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{get_auth_client, TEST_USER_PASSWD};
    use anyhow::Result;

    #[tokio::test]
    async fn test_signup() -> Result<()> {
        let client = match get_auth_client().await {
            Ok(client) => client,
            Err(e) => {
                println!("Cannot create an auth client. Most probably SUPABASE_URL and/or SUPABASE_KEY env vars are not exported: {e}");
                return Ok(());
            }
        };

        let session = client
            .signup(
                IdType::Email("newuser@supabase.rs".into()),
                TEST_USER_PASSWD,
                None,
            )
            .await?;

        assert_eq!(
            session.user.unwrap().email,
            Some("newuser@supabase.rs".into())
        );

        Ok(())
    }
}
