//! Handles token refresh operations

use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::token::TokenResponse;
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
            return Err(AuthError::InvalidParameters("empty token".to_string()));
        }

        let token_grant = TokenRefreshGrant {
            refresh_token: token.to_string(),
        };

        let resp = match self
            .http_post("token?grant_type=refresh_token")
            .json(&token_grant)
            .send()
            .instrument(trace_span!("gotrue refresh token"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("{e:?}");
                return Err(AuthError::Http);
            }
        };

        let token_response: TokenResponse = self.handle_response_code(resp).await?.unwrap();

        debug!(
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
