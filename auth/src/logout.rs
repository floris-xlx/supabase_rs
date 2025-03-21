//! Handles user logout operations

use tracing::{error, instrument, trace_span, Instrument};

use crate::util::handle_response_code;
use crate::{AuthClient, AuthError};

impl AuthClient {
    /// Logs out a user by invalidating their token
    ///
    /// # Arguments
    /// * `token` - The access token to invalidate
    ///
    /// # Returns
    /// * `Result<(), AuthError>` - Success or error
    #[instrument(skip_all)]
    pub async fn logout(&self, token: &str) -> Result<(), AuthError> {
        let resp = match self
            .http_client
            .post(format!("{}/auth/v1/logout", self.supabase_api_url))
            .bearer_auth(token)
            .header("apiKey", &self.supabase_anon_key)
            .send()
            .instrument(trace_span!("gotrue logout user"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("{e:?}");
                return Err(AuthError::Http);
            }
        };

        let logout_response: serde_json::Value = handle_response_code(resp).await?;
        dbg!(&logout_response);
        Ok(())
    }
}
