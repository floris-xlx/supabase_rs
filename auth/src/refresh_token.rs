//! Handles token refresh operations

use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::token::TokenResponse;
use crate::util::handle_response_code;
use crate::AuthClient;

/// Request payload for refreshing an authentication token
#[derive(Debug, Serialize, Deserialize)]
struct TokenRefreshGrant {
    /// The refresh token to use
    pub refresh_token: String,
}

impl AuthClient {
    /// Refreshes an authentication token
    ///
    /// # Arguments
    /// * `token` - The refresh token to use
    ///
    /// # Returns
    /// * `Result<TokenResponse, AuthError>` - New token response or error
    #[instrument(skip(self))]
    pub async fn refresh_token(&self, token: &str) -> Result<TokenResponse, AuthError> {
        if token.is_empty() {
            error!("empty token");
            return Err(AuthError::InvalidParameters);
        }

        let token_grant = TokenRefreshGrant {
            refresh_token: token.to_string(),
        };

        let resp = match self
            .http_client
            .post(format!(
                "{}/auth/v1/{}",
                self.supabase_api_url, "token?grant_type=refresh_token"
            ))
            .bearer_auth(&self.supabase_anon_key)
            .header("apiKey", &self.supabase_anon_key)
            .json(&token_grant)
            .send()
            .instrument(trace_span!("gotrue refresh token"))
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

        let token_response = match serde_json::from_str::<TokenResponse>(&resp_text) {
            Ok(token_response) => token_response,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Internal);
            }
        };
        info!(
            tokens_are_nonempty =
                !token_response.access_token.is_empty() && !token_response.refresh_token.is_empty()
        );
        debug!(
            token = token_response.access_token,
            refresh_token = token_response.refresh_token
        );

        Ok(token_response)
    }
}
