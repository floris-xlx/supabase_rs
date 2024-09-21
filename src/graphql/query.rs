use anyhow::{Error as AnyError, Result};
use serde_json::Value;

use crate::graphql::parse::parse_outer;
use crate::graphql::utils::format_endpoint::endpoint;
use crate::graphql::utils::headers::headers;
use crate::graphql::{Query, RootTypes};
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
