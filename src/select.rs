//! ## This module contains the `select()` function
//!
//! ### Conditional filtering
//! The `select()` function allows you to filter the rows you want to retrieve from the table.
//! You can filter the rows based on the column values or their relationships.
//!
//! ### Filter operators
//! - [`eq`](#eq) - Equal to the column value
//! - [`neq`](#neq) - Not equal to the column value
//! - [`gt`](#gt) - Greater than the column value
//! - [`lt`](#lt) - Less than the column value
//! - [`gte`](#gte) - Greater than or equal to the column value
//! - [`lte`](#lte) - Less than or equal to the column value
//!
//!
//! ### Usage
//! First make sure you have initialized the Supabase Client
//! [Initalizing the SupabaseClient](#lib)
//!
//! This will return all `dog` rows where the value is `scooby` in the `animals` table
//! ```rust
//! use supabase_rs::SupabaseClient;
//! use dotenv::dotenv;
//! use std::env::var;
//! use serde_json::Value;
//!
//! async fn select_scooby(
//!      supabase_client: SupabaseClient
//! )-> Result<(), String>{
//!     let data: Result<Vec<Value>, String> = supabase_client
//!         .select("animals")
//!         .eq("dog", "scooby")
//!         .execute()
//!     .await;
//!
//! match data {
//!    Ok(data) => {
//!        println!("Data: {:?}", data);
//!        Ok(())
//!    },
//!    Err(error) => {
//!        println!("Error: {:?}", error);
//!        Err(error)
//!    }
//! }
//! ```
//! ## Counting
//! You can also count the number of rows that match the filter criteria and return it under `total_records_count`
//! 
//! ### Counting with filtering
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!    .select("animals")
//!    .eq("dog", "scooby")
//!    .count()
//!    .execute()
//!    .await;
//! ```
//! 
//! ### Counting without filtering
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!    .select("animals")
//!    .count()
//!    .execute()
//!    .await;
//! ```
//! 
//! ## Methods / Operators
//!
//! ### eq
//! This method checks if the Column is equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .eq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//! ### neq
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .neq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//! ### gt
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .gt("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### lt
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .lt("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### gte
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .gte("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### lte
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .lte("weight", "100")
//!     .execute()
//!     .await;
//! ```
//!
#![allow(clippy::inherent_to_string)]
#![allow(clippy::derivable_impls)]
#![allow(rustdoc::invalid_rust_codeblocks)]



use crate::SupabaseClient;
use reqwest;
use reqwest::Client;
use reqwest::Response;
use serde_json::{Value, json};

use crate::query::QueryBuilder;



impl SupabaseClient {
    /// Initializes a `QueryBuilder` for a specified table.
    /// 
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table to be queried.
    ///
    /// # Returns
    /// A `QueryBuilder` instance configured for the specified table.
    pub fn select(&self, table_name: &str) -> QueryBuilder {
        QueryBuilder::new(self.clone(), table_name)
    }


    
    /// Executes a query against a specified table with a given query string.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table to be queried.
    /// * `query_string` - A string slice that holds the query parameters.
    ///
    /// # Returns
    /// A `Result` which is either a vector of `Value` representing the records fetched from the database
    /// or a `String` error message in case of failure.
    ///
    /// # Errors
    /// This function will return an error if the HTTP request fails or if the server returns a non-success status code.
    pub async fn execute(&self, table_name: &str, query_string: &str) -> Result<Vec<Value>, String> {
        // Build the client and the endpoint
        let endpoint: String = format!("{}/rest/v1/{}?{}", self.url, table_name, query_string);
        let client: Client = Client::new();

        // if the endpoint ends in count=exact& then we know we are doing a count query and we should remove that part but run the first part of the if statement
        if endpoint.ends_with("count=exact&") {
            let endpoint: String = endpoint.replace("count=exact&", "");
                // Send the request
                let response: Response = match client
                .get(&endpoint)
                .header("apikey", &self.api_key)
                .header("Authorization", &format!("Bearer {}", &self.api_key))
                .header("Content-Type", "application/json")
                .header("prefer", "count=exact")
                .header("x_client_info", "supabase-rs/0.2.5")
                
           
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => return Err(error.to_string()),
            };

            handle_count_response(response).await
        
        } else {
            // Send the request
            let response: Response = match client
                .get(&endpoint)
                .header("apikey", &self.api_key)
                .header("Authorization", &format!("Bearer {}", &self.api_key))
                .header("Content-Type", "application/json")
                .header("x_client_info", "supabase-rs/0.2.4")
                
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => return Err(error.to_string()),
            };

            // Process the response
            handle_count_response(response).await
        }

    }
}


async fn handle_count_response(response: Response) -> Result<Vec<Value>, String> {
    // Extract the `headers` and `content-range` from the response
    let headers: &reqwest::header::HeaderMap = response.headers();
    let content_range_option: Option<&str> = headers.get("content-range").and_then(|v| v.to_str().ok());
    
    // Initialize total_records to None
    let mut total_records: Option<i32> = None;

    // If content-range header exists, parse the total records
    if let Some(content_range) = content_range_option {
        total_records = content_range.split('/').nth(1).and_then(|v| v.parse::<i32>().ok());
    }

    // Process the response
    if response.status().is_success() {
        let mut records: Vec<Value> = response.json::<Vec<Value>>().await.unwrap();
        if let Some(count) = total_records {

            // Add total_records to the records if available
            records.push(json!({"total_records_count": count}));
        }
        Ok(records)
    } else {

        Err(response.status().to_string())
    }
}