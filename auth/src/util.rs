//! Utility functions for handling HTTP responses and other common operations

use crate::AuthError;
use log::{debug, error, warn};
use reqwest::{Response, StatusCode};
use tracing::instrument;

/// Handles HTTP response status codes and maps them to appropriate AuthErrors
///
/// # Arguments
/// * `resp_status` - The HTTP status code to evaluate
///
/// # Returns
/// * `Result<(), AuthError>` - Ok if status is successful, appropriate error otherwise
#[instrument]
pub(super) async fn handle_response_code<T>(http_response: Response) -> Result<T, AuthError>
where
    T: serde::de::DeserializeOwned,
{
    let status = http_response.status();
    debug!("response.status = {}", status);
    if http_response.status().is_success() {
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

        Ok(t)
    } else {
        warn!("response.text = {}", &http_response.text().await.unwrap());
        match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(AuthError::NotAuthorized),
            StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
                Err(AuthError::InvalidParameters)
            }
            StatusCode::NOT_ACCEPTABLE => Err(AuthError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR | _ => Err(AuthError::GeneralError),
        }
    }
}
