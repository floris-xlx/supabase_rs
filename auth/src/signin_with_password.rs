use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::token::TokenResponse;
use crate::IdType;
use crate::{AuthClient, AuthSession};

#[derive(Debug, Deserialize, Serialize)]
struct TokenPasswordGrant {
    email: Option<String>,
    phone: Option<String>,
    password: String,
}

impl AuthClient {
    #[instrument(skip_all)]
    pub async fn signin_with_password(
        &self,
        id: IdType,
        password: &str,
    ) -> Result<AuthSession, AuthError> {
        if password.is_empty() {
            error!("empty password");
            return Err(AuthError::InvalidParameters);
        }

        let token_password_grant = match id {
            IdType::Email(email) => {
                if email.is_empty() {
                    error!("empty email");
                    return Err(AuthError::InvalidParameters);
                }

                debug!("email = {email}");
                TokenPasswordGrant {
                    email: Some(email),
                    phone: None,
                    password: password.to_string(),
                }
            }
            IdType::PhoneNumber(phone_number) => {
                if phone_number.is_empty() {
                    error!("empty phone_number");
                    return Err(AuthError::InvalidParameters);
                }

                debug!("phone_number = {phone_number}");
                TokenPasswordGrant {
                    email: None,
                    phone: Some(phone_number),
                    password: password.to_string(),
                }
            }
        };

        let resp = match self
            .http_post("token?grant_type=password")
            .json(&token_password_grant)
            .send()
            .instrument(trace_span!("gotrue token password"))
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
            "tokens_are_nonempty = {}",
            !token_response.access_token.is_empty() && !token_response.refresh_token.is_empty()
        );
        debug!(
            "token = {}. refresh_token = {}",
            token_response.access_token, token_response.refresh_token
        );

        let session = AuthSession {
            access_token: token_response.access_token,
            expires_in: token_response.expires_in,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            user: token_response.user,
        };

        *self.session.borrow_mut() = Some(session.clone());

        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{get_auth_client, TEST_USER_EMAIL, TEST_USER_PASSWD};
    use anyhow::Result;

    #[tokio::test]
    async fn test_signin_with_password() -> Result<()> {
        let client = match get_auth_client().await {
            Ok(client) => client,
            Err(e) => {
                println!("Cannot create an auth client. Most probably SUPABASE_URL and/or SUPABASE_KEY env vars are not exported: {e}");
                return Ok(());
            }
        };

        let session = client
            .signin_with_password(IdType::Email(TEST_USER_EMAIL.into()), TEST_USER_PASSWD)
            .await?;

        assert_eq!(session.user.unwrap().email, Some(TEST_USER_EMAIL.into()));

        Ok(())
    }
}
