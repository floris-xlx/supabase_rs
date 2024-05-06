//! ## Insert Operations
//!
//! This module provides functionalities to insert new rows into a Supabase table.
//! It leverages the Supabase REST API for performing these operations.
//!
//! ## Features
//!
//! - **Insert**: Add new rows to a table.
//! - **Insert if Unique**: Add a new row only if it does not violate a UNIQUE constraint.
//!
//! ## Usage
//!
//! Before using these operations, ensure you have a valid `SupabaseClient` instance.
//! You can then use the `insert` or `insert_if_unique` methods provided by the client to perform the desired operation.
//!
//! ### Insert Example
//!
//! ```ignore
//! let client = SupabaseClient::new(
//!     "your_supabase_url", "your_supabase_key"
//! );
//! let insert_result = client.insert(
//!     "your_table_name", json!({"column_name": "value"})
//! ).await;
//! ```
//!
//! ### Insert if Unique Example
//!
//! ```ignore
//! let client = SupabaseClient::new(
//!     "your_supabase_url", "your_supabase_key"
//! );
//! let unique_insert_result = client.insert_if_unique(
//!     "your_table_name", json!({"unique_column_name": "unique_value"})
//! ).await;
//! ```
//!
//! ## Error Handling
//!
//! Both `insert` and `insert_if_unique` methods return a `Result<String, String>`, where `Ok(String)` contains the ID of the inserted row,
//! and `Err(String)` contains an error message in case of failure.

use crate::{
    SupabaseClient,
    generate_random_id
};
use serde_json::{
    json,
    Value
};
use reqwest::{
    Client,
    Response
};

impl SupabaseClient {
    /// Inserts a new row into the specified table.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table.
    /// * `body` - A JSON value containing the data to be inserted.
    ///
    /// # Example
    /// ```rust
    /// // Initialize the Supabase client
    /// let client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// 
    /// // This will insert a new row into the table
    /// let insert_result = client.insert(
    ///   "your_table_name",
    ///   json!(
    ///     {"column_name": "value"}
    ///   )
    /// ).await;
    /// ``` 
    /// 
    /// 
    /// # Returns
    /// This method returns a `Result<String, String>`. On success, it returns `Ok(String)` with the new row's ID,
    /// and on failure, it returns `Err(String)` with an error message.
    pub async fn insert(
        &self,
        table_name: &str,
        mut body: Value
    ) -> Result<String, String> {

        let endpoint: String = format!("{}/rest/v1/{}", self.url, table_name);
        let client: Client = Client::new();
        let new_id: i64 = generate_random_id();
        body["id"] = json!(new_id);

        println!("Inserting row with body: {}", body);

        let response: Response = match client
            .post(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .header("x_client_info", "supabase-rs/0.2.6")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(e) => return Err(e.to_string())
            };

        if response.status().is_success() {
            Ok(new_id.to_string())

        } else if response.status().as_u16() == 409 {
            println!("\x1b[31mError 409: Duplicate entry. The value you're trying to insert may already exist in a column with a UNIQUE constraint.\x1b[0m");

            return Err("\x1b[31mError 409: Duplicate entry. The value you're trying to insert may already exist in a column with a UNIQUE constraint.\x1b[0m".to_string());
        } else {
            println!("Error: {:?}", response);
            return Err(response.status().to_string())
        }
    }

    /// Inserts a row into the specified table if the value is unique and does not exist in the table already.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table.
    /// * `body` - A JSON value containing the data to be inserted.
    ///
    /// ## Example
    /// ```rust
    /// // Initialize the Supabase client
    /// let client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    /// 
    /// // This will insert a new row into the table if the value is unique
    /// let unique_insert_result = client.insert_if_unique(
    ///    "your_table_name",
    ///   json!({"unique_column_name": "unique_value"})
    /// ).await;
    /// ``` ```
    /// 
    /// 
    /// # Returns
    /// This method returns a `Result<String, String>`. On success, it returns `Ok(String)` with the new row's ID,
    /// and on failure, it returns `Err(String)` with an error message indicating a duplicate entry.
    pub async fn insert_if_unique(
        &self,
        table_name: &str,
        body: Value
    ) -> Result<String, String> {
        let conditions: &serde_json::Map<String, Value> = body.as_object().unwrap();
    
        // Check if any row in the table matches all the column-value pairs in the body
        let mut query: crate::query::QueryBuilder = self.select(table_name);
        for (
            column_name, 
            column_value
        ) in conditions {
            // turn column_value into a string before passing it to the query
            // ONLY if it's NOT a string
            let column_value_str: String = match column_value {
                Value::String(s) => s.clone(),
                _ => column_value.to_string(),
            };


            // our query is sensitive to the type of the column value
            query = query.eq(
                column_name, 
                column_value_str.as_str()
            );
        }
    
        let response: Result<Vec<Value>, String> = query.execute().await;
    
        // If no existing row matches all conditions, proceed with the insert
        if let Ok(results) = response {
            if results.is_empty() {
                return self.insert(table_name, body).await;
            }
        } else {
            return Err("Failed to execute select query".to_string());
        }
    
        Err("Error 409: Duplicate entry. The values you're trying to insert may already exist in a column with a UNIQUE constraint".to_string())
    }
}