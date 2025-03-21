use anyhow::Result;

use crate::graphql::parse::parse_outer;
use crate::graphql::Query;

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
