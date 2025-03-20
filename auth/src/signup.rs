//! Handles user signup operations

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::user::UserSchema;
use crate::util::handle_response_code;
use crate::{AuthClient, IdType};

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
        password: String,
        _metadata: Option<HashMap<String, String>>,
    ) -> Result<(UserSchema, String), AuthError> {
        let body = match signup_id_type {
            IdType::Email(email) => SignupRequest {
                email: Some(email),
                phone_number: None,
                password,
                data: _metadata,
            },
            IdType::PhoneNumber(phone_number) => SignupRequest {
                email: None,
                phone_number: Some(phone_number),
                password,
                data: _metadata,
            },
        };

        let resp = match self
            .http_client
            .post(format!("{}/auth/v1/{}", self.supabase_api_url, "signup"))
            .header("apiKey", &self.supabase_anon_key)
            .bearer_auth(&self.supabase_anon_key)
            .json(&body)
            .send()
            .instrument(trace_span!("gotrue create user"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };

        let resp_code_result = handle_response_code(resp.status()).await;
        let resp_text = match resp.text().await {
            Ok(resp_text) => resp_text,
            Err(e) => {
                log::error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result?;

        let created_user_resp = match serde_json::from_str::<SignupResponse>(&resp_text) {
            Ok(token_response) => token_response,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Internal);
            }
        };

        let created_user = created_user_resp.user;
        info!(user_id = created_user.id.to_string(), "created user");

        Ok((created_user, created_user_resp.access_token))
    }
}
