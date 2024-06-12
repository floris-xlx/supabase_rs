use anyhow::{Result, Error as AnyError};
use serde_json::Value;

use crate::graphql::{RootTypes, Query};
use crate::graphql::utils::format_endpoint::endpoint;
use crate::graphql::utils::headers::headers;
use crate::graphql::parse::parse_outer;
use crate::SupabaseClient;


impl Query {
    pub async fn verify(&self) -> Result<bool> {
        Ok(parse_outer(&self.query))
    }
}