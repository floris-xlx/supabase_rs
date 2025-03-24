//! Handles user logout operations

use tracing::{debug, error, instrument, trace_span, Instrument};

use crate::{AuthClient, AuthError};

#[derive(Debug, serde::Serialize)]
pub enum SignOutScope {
    GLOBAL,
    LOCAL,
    OTHERS,
}

impl AuthClient {
    /// Logs out a user by invalidating their token
    ///
    /// # Arguments
    /// * `token` - The access token to invalidate
    ///
    /// # Returns
    /// * `Result<(), AuthError>` - Success or error
    #[instrument(skip_all)]
    pub async fn signout(&self) -> Result<(), AuthError> {
        self.signout_with_scope(SignOutScope::GLOBAL).await
    }

    #[instrument(skip_all)]
    pub async fn signout_with_scope(&self, scope: SignOutScope) -> Result<(), AuthError> {
        match self.session.borrow().as_ref() {
            Some(session) => {
                let body = serde_json::to_string(&scope)?;
                let resp = match self
                    .http_client
                    .post(format!("{}/auth/v1/logout", self.supabase_api_url))
                    .bearer_auth(session.access_token.clone())
                    .header("apiKey", &self.supabase_anon_key)
                    .body(body)
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

                self.handle_response_code::<String>(resp).await?;
            }
            None => {
                debug!("no session found");
            }
        }
        *self.session.borrow_mut() = None;
        Ok(())
    }
}
