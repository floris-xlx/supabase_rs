//! Success response types.
//!
//! This is still a wip
//!

use reqwest::Response;
use serde_json::Value;

use crate::errors::{
    api_key_missing_error, authorization_failed_error, invalid_query_error, unknown_error,
    ErrorSupabase,
};

/// Handles the response from the Supabase API.
pub async fn handle_response(response: Response) -> Result<Vec<Value>, String> {
    if response.status().is_success() {
        match response.json::<Vec<Value>>().await {
            Ok(records) => Ok(records),
            Err(error) => Err(error.to_string()),
        }
    } else {
        let error_message = match response.status().as_u16() {
            401 => authorization_failed_error()
                .await
                .map_err(|e| e.to_string()),
            403 => api_key_missing_error().await.map_err(|e| e.to_string()),
            400 => invalid_query_error().await.map_err(|e| e.to_string()),
            _ => unknown_error().await.map_err(|e| e.to_string()),
        };

        // Convert the error to the expected type
        Err(error_message.unwrap_err())
    }
}
