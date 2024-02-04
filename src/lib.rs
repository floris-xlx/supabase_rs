//! Supabase SDK for Rust
//!
//! This is an unofficial Rust SDK for [Supabase](https://supabase.io/), since there is no official SDK for Rust yet.
//!
//! ## Cargo.toml
//! ```toml
//! [dependencies]
//! supabase-rs = "0.2.0"
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
//!     dotenv().ok();
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
//! ## Tree of contents
//! - [Update](#update)
//! - [Select](#select)
//!
//! ## Update
//! I'll be adding more methods and enriching the SDK over the next few days, for now!

use rand::Rng;

pub mod update;
pub mod select;
pub mod insert;


// #[derive(Debug)]
#[derive(Clone)]
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
