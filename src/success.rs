//! Success response types.
//!
//! This is still a wip
//!

use reqwest::Response;
use serde_json::{Value, json};

use crate::errors::{
    api_key_missing_error, authorization_failed_error, invalid_query_error, unknown_error,
    ErrorSupabase,
};

/// Handles the response from the Supabase API.
pub async fn handle_response(response: Response) -> Result<Vec<Value>, String> {
    if response.status().is_success() {
        let headers: &reqwest::header::HeaderMap = response.headers();
        let content_range_option: Option<&str> =
            headers.get("content-range").and_then(|v| v.to_str().ok());

        let mut total_records: Option<i32> = None;

        if let Some(content_range) = content_range_option {
            total_records = content_range
                .split('/')
                .nth(1)
                .and_then(|v| v.parse::<i32>().ok());
        }

        let mut records: Vec<Value> = match response.json::<Vec<Value>>().await {
            Ok(records) => records,
            Err(error) => return Err(error.to_string()),
        };

        if let Some(count) = total_records {
            records.push(json!({"total_records_count": count}));
        }
        Ok(records)
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
