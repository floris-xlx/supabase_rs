//! Handles user retrieval operations

use crate::models::user::UserSchema;
use crate::{AuthClient, AuthError};
use tracing::{error, trace_span, Instrument};

impl AuthClient {
    /// Gets the current user details if there is an existing session, or None if not.
    ///
    /// # Returns
    /// * `Result<Option<UserSchema>, AuthError>` - User data if found, None if not found, or error
    pub async fn get_user(&self) -> Result<Option<UserSchema>, AuthError> {
        match self.session.borrow().as_ref() {
            Some(session) => {
                let user = session.user.clone();
                Ok(user)
            }
            None => Ok(None),
        }
    }

    /// Gets the current user details from the backend if there is an existing session, or None if not.
    ///
    /// # Arguments
    ///
    /// # Returns
    /// * `Result<Option<UserSchema>, AuthError>` - User data if found, None if not found, or error
    pub async fn get_user_remote(&self) -> Result<Option<UserSchema>, AuthError> {
        let access_token = match self.session.borrow().as_ref() {
            Some(session) => session.access_token.clone(),
            None => return Err(AuthError::NotAuthorized),
        };

        let resp = match self
            .http_client
            .get(format!("{}/auth/v1/user", self.supabase_api_url))
            .bearer_auth(access_token)
            .header("apiKey", &self.supabase_anon_key)
            .send()
            .instrument(trace_span!("gotrue get_user"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("get_user() request failed: {e}");
                return Err(AuthError::Http);
            }
        };

        if resp.status().is_success() {
            let user: UserSchema = match resp.json().await {
                Ok(user) => user,
                Err(e) => {
                    error!("{e}");
                    return Err(AuthError::NotAuthorized);
                }
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{get_auth_client, TEST_USER_EMAIL, TEST_USER_PASSWD};
    use crate::IdType;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_user() -> Result<()> {
        let client = match get_auth_client().await {
            Ok(client) => client,
            Err(e) => {
                println!("Cannot create an auth client. Most probably SUPABASE_URL and/or SUPABASE_KEY env vars are not exported: {e}");
                return Ok(());
            }
        };

        let _session = client
            .signin_with_password(IdType::Email(TEST_USER_EMAIL.into()), TEST_USER_PASSWD)
            .await?;

        let user = client.get_user().await?;

        assert_eq!(user.unwrap().email, Some(TEST_USER_EMAIL.into()));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_remote() -> Result<()> {
        let client = match get_auth_client().await {
            Ok(client) => client,
            Err(e) => {
                println!("Cannot create an auth client. Most probably SUPABASE_URL and/or SUPABASE_KEY env vars are not exported: {e}");
                return Ok(());
            }
        };

        let _session = client
            .signin_with_password(IdType::Email(TEST_USER_EMAIL.into()), TEST_USER_PASSWD)
            .await?;

        let user = client.get_user_remote().await?;

        assert_eq!(user.unwrap().email, Some(TEST_USER_EMAIL.into()));

        Ok(())
    }
}
