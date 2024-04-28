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
            .header("x_client_info", "supabase-rs/0.2.5")
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
        mut body: Value
    ) -> Result<String, String> {
        let column_name: String = body
            .as_object_mut()
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .to_string();

        let column_value = match body
            .as_object_mut()
            .unwrap()
            .get(&column_name)
            .unwrap() {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => panic!("Unsupported type for column value"),
            };

        let response: Result<Vec<Value>, String> = self
            .select(table_name)
            .eq(&column_name, &column_value)
            .execute()
            .await;

        if response.unwrap().is_empty() {
            return self.insert(table_name, body).await;
        }

        Err("\x1b[31mError 409: Duplicate entry. The value you're trying to insert may already exist in a column with a UNIQUE constraint.\x1b[0m".to_string())
    }
}