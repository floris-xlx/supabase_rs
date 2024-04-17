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
//! supabase-rs = "0.2.3"
//! 
//! // With the [storage] feature
//! supabase-rs = { version = "0.2.3", features = ["storage"] }
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

pub mod update;
pub mod select;
pub mod insert;
pub mod storage;



#[derive(Debug, Clone)]
pub struct SupabaseClient {
    pub url: String,
    pub api_key: String
}


impl SupabaseClient {
    // service role and anon key will be cooked here
    pub fn new(
        supabase_url: String,
        private_key: String
    ) -> Self {

        Self {
            url: supabase_url,
            api_key: private_key
        }
    }
}

/// Generates a random 64-bit signed integer within a larger range
pub fn generate_random_id() -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..i64::MAX)
}
