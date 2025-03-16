//! # Supabase SDK for Rust
//!
//! This is an unofficial Rust SDK for [Supabase](https://supabase.io/), since there is no official SDK for Rust yet.
//!
//! ## Features
//! - [**`Insert`**](#insert): Add new rows to a table.
//! - [**`Insert if unique`**](#insert-if-unique): Add a new row only if it does not violate a UNIQUE constraint.
//! - [**`Update`**](#update): Modify existing rows in a table based on a unique identifier.
//! - [**`Select`**](#select): Insert a new row into a table if it does not exist, or update it if it does.
//! - [**`Select with count`**](#select-with-count): Select rows from a table and count the number of rows that match the filter criteria.
//! - [**`Select with filter`**](#select-with-filter): Select rows from a table based on a filter criteria.
//! - [**`Select with filter and count`**](#selecting-with-filter-and-count): Select rows from a table based on a filter criteria and count the number of rows that match the filter criteria.
//! - [**`Delete`**](#delete): Delete a row from a table based on a unique identifier.
//!
//! ## Graphql features
//! - [**`Query request`**](#query-request): Runs a GraphQL query to supabase
//!
//! ## Feature flags
//! - **`storage`**: Enables the `Storage` module to interact with Supabase Storage.
//! - **`nightly`**: Enables the nightly features.
//! - **`rustls`**: Forces the client into using `rustls` over `OpenSSL`.
//!
//! ## Nightly Build
//! - **`nightly`**: Enables the `GraphQL` module to interact with Supabase GraphQL API.
//!
//! Nightly features are not stable and may break at any time without notice, so use with caution.
//!
//! Nightly WILL send a warning message, to disable the `nightly` warning message
//!
//! disable it in your `.env` as such:
//! ```env
//! SUPABASE_RS_NO_NIGHTLY_MSG=true
//! ```
//!
//! ## Cargo.toml
//! ```toml
//! [dependencies]
//! supabase-rs = "0.3.7"
//!
//! // With the [storage] feature
//! supabase-rs = { version = "0.3.7", features = ["storage"] }
//! ```
//!
//! ## Usage
//! First make sure you have initialized the Supabase Client
//! [Initalizing the SupabaseClient](#initialize-the-supabase-client)
//!
//! ## Authentication
//! The Supabase Client is initialized with the Supabase URL and the Supabase Key.
//! Which are environment variables that can be set in a `.env` file under the following names or any other
//! ```ignore
//! SUPABASE_URL=
//! SUPABASE_KEY=
//! ```
//!
//! ## Examples
//!
//! ### Initialize the Supabase Client
//!  ```rust
//! use supabase_rs::SupabaseClient;
//!
//! use dotenv::dotenv;
//! use std::env::var;
//!
//! async fn initialize_supabase_client() -> SupabaseClient {
//!     dotenv().ok(); // Load the .env file
//!
//!     let supabase_client: SupabaseClient = SupabaseClient::new(
//!         var("SUPABASE_URL").unwrap(),
//!         var("SUPABASE_KEY").unwrap()
//!         ).unwrap();
//!
//!         supabase_client
//!    }
//! ```
//! This will initialize the Supabase Client with the Supabase URL and the Supabase Key, and return the Supabase Client to be passed to other methods.
//!
//! ### Insert
//! This will insert a new row into the `test` table with the value `value_test` in the `dog` column.
//!
//! ```rust,ignore
//! // i know the imports are self explanatory but it makes it easier for beginners:)
//! use serde_json::json;
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!     "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn insert_example(
//!    client: SupabaseClient
//! ) -> Result<(), String> {
//!     let insert_result = client
//!         .insert(
//!             "test",
//!             json!({
//!                 "dog": "value_test"
//!             }),
//!        ).await;
//! ```
//!
//! ### Insert if unique
//! This will insert a new row into the `test` table with the value `value_test` in the `dog` column if the value is unique.
//! It's a drop-in replacement for `insert` without relying on Supabase's unique constraints on the database.
//!
//! ```rust,ignore
//! use serde_json::json;
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!     "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn insert_example(
//!    client: SupabaseClient
//! ) -> Result<(), String> {
//!     let insert_result = client
//!         .insert_if_unique(
//!             "test",
//!             json!({
//!                 "dog": "value_test"
//!             }),
//!        ).await;
//! ```
//!
//! ### Update
//! This will update the row in the `test` table with the value `value_test` in the `dog` column where the `id` is `1`.
//!
//! ```rust,ignore
//! use serde_json::json;
//! use supabase_rs::SupabaseClient;
//!
//! let client = SupabaseClient::new(
//!    "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn update_example(
//!   client: SupabaseClient
//! ) -> Result<(), String> {
//!    let update_result = client
//!       .update_with_column_name(
//!          "table_name", // the table name
//!          "column_name",    // the column name to filter by
//!          "id", // the value to filter by (can be any value to use as key)
//!          json!({
//!            "dog": "value_test"  // the new value
//!          }),
//!      ).await;
//! ```
//!
//! ### Select
//! This will return all `dog` rows where the value is `scooby` in the `animals` table
//!
//! ```rust,ignore
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!    "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn select_scooby(
//!    supabase_client: SupabaseClient
//! ) -> Result<(), String> {
//!
//! let data: Result<Vec<Value>, String> = supabase_client
//!    .select("animals")
//!    .eq("dog", "scooby")
//!    .execute()
//!    .await;
//! ```
//!
//! ### Select on specific column
//! This will return all the `dog` rows where the value is `scooby` in the `animals` table and only return the `dog` column.
//!
//! ```rust,ignore
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!    "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn select_scooby(
//!    supabase_client: SupabaseClient
//! ) -> Result<(), String> {
//!
//! let data: Result<Vec<Value>, String> = supabase_client
//!    .select("animals")
//!    .columns(["dog"].to_vec())
//!    .eq("dog", "scooby")
//!    .execute()
//!    .await;
//! ```
//!
//! ### Select with Count
//! <div class="warning">Counting is very expensive and will be alot slower, so only use it if you need it </div>
//!
//! This will return all `dog` rows where the value is `scooby` in the `animals` table and count the number of rows that match the filter criteria.
//!
//! ```rust,ignore
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!   "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn select_scooby_with_count(
//!   supabase_client: SupabaseClient
//! ) -> Result<(), String> {
//!  let data: Result<Vec<Value>, String> = supabase_client
//!    .select("animals")
//!    .count()
//!    .execute()
//!    .await;
//! ```
//!
//! ### Select with Filter
//! This will return all `dog` rows where the value is `scooby` in the `animals` table
//!
//! ```rust,ignore
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!   "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn select_scooby_with_filter(
//!  supabase_client: SupabaseClient
//! ) -> Result<(), String> {
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .eq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//!
//! ### Selecting with Filter and Count
//! <div class="warning">Counting is very expensive and will be alot slower, so only use it if you need it </div>
//!
//! This will return all `dog` rows where the value is `scooby` in the `animals` table and count the number of rows that match the filter criteria.
//! ```rust,ignore
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!  "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn select_scooby_with_filter_and_count(
//! supabase_client: SupabaseClient
//! ) -> Result<(), String> {
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .eq("dog", "scooby")
//!     .count()
//!     .execute()
//!     .await;
//! ```
//!
//! ### Delete
//! This will delete the row in the `test` table where the `id` is `1`.
//!
//! ```rust,ignore
//! // i know the imports are self explanatory but it makes it easier for beginners:)
//! use supabase_rs::SupabaseClient;
//!
//! // always pass an initialized SupabaseClient to the method
//! let client = SupabaseClient::new(
//!   "your_supabase_url", "your_supabase_key"
//! );
//!
//! async fn delete_example(
//!  client: SupabaseClient
//! ) -> Result<(), String> {
//! let delete_result = client
//!     .delete("test", "1")
//!     .await;
//! ```
//!
//! //! <div class="warning">Experimental features, Not ready for prod!</div>
//!
//!
//! ### Get ID by Column, Cell values
//! This will return the ID of the row in the specified table where the column matches the provided email.
//!
//! ```rust,ignore
//! #[tokio::main]
//! async fn main() {
//!     // Initialize the Supabase Client
//!     let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
//!
//!     let email = "example@email.com".to_string();
//!     let table_name = "users".to_string();
//!     let column_name = "email".to_string();
//!     match supabase_client.get_id(email, table_name, column_name).await {
//!         Ok(id) => println!("Found ID: {}", id),
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```
//!
//!
//! ## Different Operations
//! - [Insert](./insert/index.html)
//! - [Update](./update/index.html)
//! - [Select](./select/index.html)
//! - [Storage](./storage/index.html)
//! - [Realtime](./realtime/index.html)
//! - [Query](./query/index.html)
//! - [Errors](./errors/index.html)
//! - [Success](./success/index.html)
//! - [Tests](./tests/index.html)
//! - [GraphQL](./graphql/index.html)
//!
//!
//! ## Update
//! I'll be adding more methods and enriching the SDK over the next few weeks, for now!
//!
//! ## Contributers
//!

use rand::prelude::ThreadRng;
use rand::Rng;
use reqwest::Client;

pub mod delete;
pub mod errors;
pub mod insert;
pub mod query;
pub mod query_builder;
pub mod request;
pub mod routing;
pub mod select;
pub mod success;
pub mod tests;
pub mod update;
pub mod type_gen;


pub mod graphql;
pub mod nightly;

// This is locked by feature flag `storage` & `realtime`
pub mod realtime;
pub mod storage;

use errors::Result;

/// A client structure for interacting with Supabase services.
///
/// This structure holds the necessary details to make requests to the Supabase API.
/// It contains the base URL of the Supabase project and the API key for authentication.
///
/// # Fields
/// - `url`: The base URL of the Supabase project.
/// - `api_key`: The API key used for authenticating requests to Supabase.
#[derive(Debug, Clone)]
pub struct SupabaseClient {
    url: String,
    api_key: String,
    client: reqwest::Client,
}

impl SupabaseClient {
    /// Creates a new instance of `SupabaseClient` using the provided Supabase URL and private API key.
    ///
    /// This function is crucial for setting up the client with the necessary credentials to interact with Supabase services.
    /// The `supabase_url` should point to your Supabase project URL, and the `private_key` should be your secret API key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use supabase_rs::SupabaseClient;
    /// let client = SupabaseClient::new(
    ///     "https://your-project.supabase.co".to_string(),
    ///     "your-secret-key".to_string(),
    /// );
    /// ```
    pub fn new(supabase_url: String, private_key: String) -> Result<Self> {
        #[cfg(feature = "rustls")]
        let client = Client::builder().use_rustls_tls().build()?;

        #[cfg(not(feature = "rustls"))]
        let client = Client::new();

        Ok(Self {
            url: supabase_url,
            api_key: private_key,
            client,
        })
    }
}

/// Generates a random 64-bit signed integer within a larger range
pub fn generate_random_id() -> i64 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..i64::MAX)
}
pub mod supabase_types;
