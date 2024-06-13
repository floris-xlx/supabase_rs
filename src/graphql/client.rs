//! # GraphQL Client 
//! 
//! The GraphQL client is used to send GraphQL queries to the Supabase API.

use crate::SupabaseClient;

// local imports
use crate::graphql::Query;
use crate::graphql::RootTypes;
use crate::graphql::utils::format_endpoint::endpoint;
