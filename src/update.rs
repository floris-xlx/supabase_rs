//! ## Update and Upsert Operations
//!
//! This module provides functionalities to update or upsert (update or insert) rows in a Supabase table.
//! It leverages the Supabase REST API for performing these operations.
//!
//! ## Features
//!
//! - **Update**: Modify existing rows in a table based on a unique identifier.
//! - **Upsert**: Insert a new row into a table if it does not exist, or update it if it does.
//!
//! ## Usage
//!
//! Before using these operations, ensure you have a valid `SupabaseClient` instance.
//! You can then use the `update` or `upsert` methods provided by the client to perform the desired operation.
//!
//! ### Update Example
//!
//! ```
//! # use supabase_rs::SupabaseClient;
//! #[tokio::main]
//! async fn main() {
//!     let client = SupabaseClient::new(
//!         "your_supabase_url".to_string(), "your_supabase_key".to_string()
//!     ).unwrap();
//!     let update_result = client.from("your_table_name").update(
//!         "row_id", serde_json::json!({"column_name": "new_value"})
//!     ).await;
//! }
//! ```
//!
//! ### Upsert Example
//!
//! ```
//! # use supabase_rs::SupabaseClient;
//! #[tokio::main]
//! async fn main() {
//!     let client = SupabaseClient::new(
//!         "your_supabase_url".to_string(), "your_supabase_key".to_string()
//!     ).unwrap();
//!     let upsert_result = client.from("your_table_name").upsert(
//!         "row_id", serde_json::json!({"column_name": "value"})
//!     ).await;
//! }
//! ```
//!
//! ## Error Handling
//!
//! Both `update` and `upsert` methods return a `Result<(), String>`, where `Ok(())` indicates a successful operation,
//! and `Err(String)` contains an error message in case of failure.
use crate::request::headers::HeadersTypes;
use crate::FromTable;
use reqwest::Response;
use serde_json::{json, Value};

impl<'s> FromTable<'s> {
    /// Updates a row in the table, based on the id
    pub async fn update(&self, id: &str, body: Value) -> Result<String, String> {
        Self::update_with_column_name(self, "id", id, body).await
    }

    /// Updates a row in the table, based on the column name
    pub async fn update_with_column_name(
        &self,
        column_name: &str,
        id: &str,
        body: Value,
    ) -> Result<String, String> {
        // endpoint and client construction
        let endpoint: String = format!("{}?{column_name}=eq.{id}", self.endpoint());

        let response: Response = match self
            .http_client
            .patch(&endpoint)
            .header(HeadersTypes::ApiKey, self.api_key)
            .header(
                HeadersTypes::Authorization,
                format!("Bearer {}", self.api_key),
            )
            .header(HeadersTypes::ContentType, "application/json")
            .header(HeadersTypes::ClientInfo, &crate::client_info())
            .body(body.to_string())
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        if response.status().is_success() {
            Ok(id.to_string())
        } else {
            Err(response.status().to_string())
        }
    }

    /// Creates a row in the table, or updates if the id already exists
    pub async fn upsert(&self, id: &str, mut body: Value) -> Result<String, String> {
        body["id"] = json!(id);
        match self.upsert_without_defined_key(body).await {
            Ok(_) => Ok(id.to_string()),
            Err(e) => Err(e),
        }
    }

    /// Creates a row in the table, or updates if the row already exists
    ///
    /// This method does not require a defined key in the body unlike the `upsert` method.
    pub async fn upsert_without_defined_key(&self, body: Value) -> Result<(), String> {
        let endpoint: String = self.endpoint();

        #[cfg(feature = "nightly")]
        use crate::nightly::print_nightly_warning;
        #[cfg(feature = "nightly")]
        print_nightly_warning();

        let response: Response = match self
            .http_client
            .post(&endpoint)
            .header(HeadersTypes::ApiKey, self.api_key)
            .header(
                HeadersTypes::Authorization,
                format!("Bearer {}", &self.api_key),
            )
            .header(HeadersTypes::ContentType, "application/json")
            .header(HeadersTypes::ClientInfo, &crate::client_info())
            .header("Prefer", "resolution=merge-duplicates")
            .header("Prefer", "return=representation")
            .body(body.to_string())
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => return Err(e.to_string()),
        };

        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }
}
