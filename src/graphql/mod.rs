#![cfg(feature = "graphql")]

pub mod client;
pub mod request;
pub mod parse;
pub mod utils;
pub mod query;
pub mod error_types;

use crate::SupabaseClient;
use serde_json::Value;

//// #### Query
#[derive(Debug)]
pub struct Query {
    pub query: Value,
}


/// #### RootTypes
#[derive(Debug)]
pub enum RootTypes {
    Query,
    Mutation,
    Subscription,
    Fragment
}