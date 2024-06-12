use anyhow::{Result, Error};
use serde_json::Value;

pub fn parse_outer(query: &Value) -> bool {
    if let Some(query_str) = query.as_str() {
        let start_1: &str = r#"{"#;
        let end_1: &str = "}";

        query_str.starts_with(start_1) && query_str.ends_with(end_1)

        
    } else {

        true
    }
}
