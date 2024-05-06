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
//! ```ignore
//! let client = SupabaseClient::new(
//!     "your_supabase_url", "your_supabase_key"
//! );
//! let update_result = client.update(
//!     "your_table_name", "row_id", json!({"column_name": "new_value"})
//! ).await;
//! ```
//!
//! ### Upsert Example
//!
//! ```ignore
//! let client = SupabaseClient::new(
//!     "your_supabase_url", "your_supabase_key"
//! );
//! let upsert_result = client.upsert(
//!     "your_table_name", "row_id", json!({"column_name": "value"})
//! ).await;
//! ```
//!
//! ## Error Handling
//!
//! Both `update` and `upsert` methods return a `Result<(), String>`, where `Ok(())` indicates a successful operation,
//! and `Err(String)` contains an error message in case of failure.
//!
use crate::SupabaseClient;
use serde_json::{
    json,
    Value
};
use reqwest::{
    Client,
    Response
};


impl SupabaseClient {
    /// Updates a row in the table, based on the id
    pub async fn update(
        &self,
        table_name: &str,
        id: &str,
        body: Value
    ) -> Result<(), String> {

        // endpoint and client construction
        let endpoint: String = format!(
            "{}/rest/v1/{}?id=eq.{}",
            self.url, table_name, id
        );
        let client: Client = Client::new();

        let response: Response = match client
            .patch(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(error) => return Err(error.to_string())
            };
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }


    /// Creates a row in the table, or updates if the id already exists
    pub async fn upsert(
        &self,
        table_name: &str,
        id: &str,
        mut body: Value
    ) -> Result<String, String> {

        let endpoint: String = format!("{}/rest/v1/{}", self.url, table_name);
        let client: Client = Client::new();
    
        body["id"] = json!(id);

        println!("Inserting row with body: {}", body);

        let response: Response = match client
            .post(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .header("x_client_info", "supabase-rs/0.2.7")
            .header("Prefer", "resolution=merge-duplicates")
            .header("Prefer", "return=representation")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(e) => return Err(e.to_string())
            };

        if response.status().is_success() {
            Ok(id.to_string())

        } else {
            Err(response.status().to_string())
        }
    }
}