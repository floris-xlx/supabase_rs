//! # Supabase SDK for Rust
//!
//! This is an unofficial Rust SDK for [Supabase](https://supabase.io/), since there is no official SDK for Rust yet.
//!
//! ## Features
//! - **Insert**: Insert rows into a Supabase table.
//! - **Update**: Modify existing rows in a table based on a unique identifier.
//! - **Select**: Retrieve rows from a table based on specific conditions.
//! - **Storage**: Interact with Supabase Storage to download and save files.
//!
//! ## Feature flags
//! - **`storage`**: Enables the `Storage` module to interact with Supabase Storage.
//!
//! ## Cargo.toml
//! ```toml
//! [dependencies]
//! supabase-rs = "0.2.4"
//!
//! // With the [storage] feature
//! supabase-rs = { version = "0.2.4", features = ["storage"] }
//! ```
//!
//! ## Usage
//! First make sure you have initialized the Supabase Client
//! [Initalizing the SupabaseClient](#initialize-the-supabase-client)
//!
//! ## Authentication
//! The Supabase Client is initialized with the Supabase URL and the Supabase Key.
//! Which are environment variables that can be set in a `.env` file under the following names or any other
//! ```
//! SUPABASE_URL=
//! SUPABASE_KEY=
//! ```
//!
//! ## Initialize the Supabase Client
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
//!         );
//!
//!         supabase_client
//!    }
//! ```
//! This will initialize the Supabase Client with the Supabase URL and the Supabase Key, and return the Supabase Client to be passed to other methods.
//!
//! ## Different Operations
//! - [Insert](./insert/index.html)
//! - [Update](./update/index.html)
//! - [Select](./select/index.html)
//! - [Storage](./storage/index.html)
//!
//! ## Update
//! I'll be adding more methods and enriching the SDK over the next few days, for now!

use rand::Rng;

pub mod delete;
pub mod insert;
pub mod select;
pub mod update;
pub mod query;
pub mod errors;
pub mod success;
pub mod tests;


// This is locked by feature flag `storage` & `realtime`
pub mod storage;
pub mod realtime;


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
    pub url: String,
    pub api_key: String,
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
    /// let client = SupabaseClient::new(
    ///     "https://your-project.supabase.co".to_string(),
    ///     "your-secret-key".to_string(),
    /// );
    /// ```
    pub fn new(supabase_url: String, private_key: String) -> Self {
        Self {
            url: supabase_url,
            api_key: private_key,
        }
    }
}

/// Generates a random 64-bit signed integer within a larger range
pub fn generate_random_id() -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..i64::MAX)
}
