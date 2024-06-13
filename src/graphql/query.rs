use anyhow::{ Result, Error as AnyError };
use serde_json::Value;

use crate::graphql::{ RootTypes, Query };
use crate::graphql::utils::format_endpoint::endpoint;
use crate::graphql::utils::headers::headers;
use crate::graphql::parse::parse_outer;
use crate::SupabaseClient;

// FIX ME: This is a temporary fix to suppress the warning
impl Query {
    /// # Verify the query
    /// 
    /// This method verifies the query to ensure it is in the correct format.
    /// 
    /// 
    pub async fn verify(&self) -> Result<bool> {
        Ok(parse_outer(&self.query))
    }
}
