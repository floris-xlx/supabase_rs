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
//! ```rust,ignore
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
//! ```rust,ignore
//! let data: Result<Vec<Value>, String> = supabase_client
//!    .select("animals")
//!    .eq("dog", "scooby")
//!    .count()
//!    .execute()
//!    .await;
//! ```
//!
//! ### Counting without filtering
//! ```rust,ignore
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
//! ```rust,ignore
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .eq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//! ### neq
//! This method checks if the Column is not equal to a value
//! ```rust,ignore
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .neq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//! ### gt
//! This method checks if the Column is not equal to a value
//! ```rust,ignore
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .gt("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### lt
//! This method checks if the Column is not equal to a value
//! ```rust,ignore
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .lt("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### gte
//! This method checks if the Column is not equal to a value
//! ```rust,ignore
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .gte("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### lte
//! This method checks if the Column is not equal to a value
//! ```rust,ignore
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

use crate::query::QueryBuilder;
use crate::request::Headers;
use crate::success::handle_response;
use crate::SupabaseClient;

use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Response};
use serde_json::Value;

#[cfg(feature = "nightly")]
use crate::nightly::print_if_dev;

impl SupabaseClient {
    /// Initializes a `QueryBuilder` for a specified table.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table to be queried.
    ///
    /// # Returns
    /// A `QueryBuilder` instance configured for the specified table.
    // #[deprecate_until(remove = ">= 0.4.x", note = "`.select()` will be deprecated. Use `.from()` to specify the table name and then use `.select()` to pass the query string. This change will align with the official Supabase documentation for other languages.")]
    pub fn select(&self, table_name: &str) -> QueryBuilder {
        QueryBuilder::new(self.clone(), table_name)
    }

    pub fn from(&self, table_name: &str) -> QueryBuilder {
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
    pub async fn execute(
        &self,
        table_name: &str,
        query_string: &str,
    ) -> Result<Vec<Value>, String> {
        // Build the client and the endpoint
        let endpoint: String = format!("{}/rest/v1/{}?{}", self.url, table_name, query_string);

        #[cfg(feature = "nightly")]
        println!("\x1b[33mEndpoint: {}\x1b[0m", endpoint);

        #[cfg(feature = "rustls")]
        let client = Client::builder().use_rustls_tls().build().unwrap();

        #[cfg(not(feature = "rustls"))]
        let client: Client = Client::new();

        #[cfg(feature = "nightly")]
        use crate::nightly::print_nightly_warning;
        #[cfg(feature = "nightly")]
        print_nightly_warning();

        let endpoint: String = if endpoint.ends_with("?count=exact") {
            endpoint.replace("?count=exact", "")
        } else {
            endpoint
        };

        // create headers with default values
        let headers: Headers = Headers::with_defaults(&self.api_key, &self.api_key);

        // convert headers to HeaderMap
        let mut header_map: HeaderMap = HeaderMap::new();
        for (key, value) in headers.get_headers() {
            header_map.insert(
                HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?,
                HeaderValue::from_str(&value).map_err(|e| e.to_string())?,
            );
        }

        // send the request
        let response: Response = match client.get(&endpoint).headers(header_map).send().await {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        // process the response
        handle_response(response).await
    }
}
