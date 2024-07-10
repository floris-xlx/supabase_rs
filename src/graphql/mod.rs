#![cfg(feature = "nightly")]

//! # GraphQL for Supabase
//! This module provides a GraphQL client for interacting with the Supabase API.
//! 
//! ## Features
//! - **Query**: Send queries to the Supabase API.
//! 
//! ## Usage
//! Before using the GraphQL client, ensure you have a valid `SupabaseClient` instance.
//! 
//! ### Example: Authenticate with Supabase
//! ```ignore
//! use supabase_rs::SupabaseClient;
//! 
//! #[tokio::main]
//! async fn main() {
//! let supabase_client: SupabaseClient = SupabaseClient::new(
//!     std::env::var("SUPABASE_URL").unwrap(),
//!     std::env::var("SUPABASE_KEY").unwrap()
//! );
//! ```
//! 
//! ### Example: Send a GraphQL query
//! ```ignore
//! let request_graphql: Request = Request::new(
//!     supabase_client,
//!     json!({
//!         "query": r#"
//!             { 
//!                 usersCollection(first: 1) { 
//!                     edges { 
//!                         node { 
//!                             user_id,
//!                             username,
//!                             email
//!                         } 
//!                     } 
//!                 } 
//!             }
//!         "#,
//!     }),
//!     supabase_rs::graphql::RootTypes::Query
//! );
//! 
//! let response: Result<serde_json::Value, anyhow::Error> = request_graphql.send().await;
//! 
//! match response {
//!     Ok(response) => println!("{:#?}", response),
//!     Err(error) => println!("{:#?}", error),
//! }}
//! ```
//!
//! 
//! 
//! ## Error Handling
//! The GraphQL client returns a `Result<Value, Error>` where `Value` is the response from the Supabase API and `Error` is an error message in case of failure.
//! 
//! ## GraphQL Query
//! The GraphQL query should be in the following format:
//! ```ignore
//! {
//!    usersCollection(first: 1) {
//!       edges {
//!         node {
//!          user_id,
//!          username,
//!          email
//!      }
//!   }
//! }
//! ```
//! *Note*: All tables in Supabase end with `Collection`. Ensure you append `Collection` to the table name.
//! 
//! ## GraphQL Mutation
//! 
//! The GraphQL mutation should be in the following format:
//! 
pub mod client;
pub mod request;
pub mod parse;
pub mod utils;
pub mod query;
pub mod error_types;
pub mod parsing;

use crate::SupabaseClient;
use serde_json::Value;

/// #### Query
/// 
/// Represents a GraphQL query.
#[derive(Debug)]
pub struct Query {
    pub query: Value,
}


/// #### RootTypes
/// 
/// The root types for GraphQL operations.
/// 
/// - `Query`: Represents a query operation.
/// - `Mutation`: Represents a mutation operation.
/// - `Subscription`: Represents a subscription operation.
/// - `Fragment`: Represents a fragment operation.
/// 
/// *Note*: Only `Query` is supported at the moment.
/// 
/// ## Example
/// 
/// ```ignore
/// use supabase_rs::graphql::RootTypes;
/// 
/// let root_type: RootTypes = RootTypes::Query;
/// 
/// println!("{:?}", root_type);
/// ```
/// 
#[derive(Debug)]
pub enum RootTypes {
    Query,
    Mutation,
    Subscription,
    Fragment
}


#[derive(Debug)]
pub enum GraphQLOperators {
    First,
    Last,
    Before,
    After,
    Filter,
    OrderBy
}