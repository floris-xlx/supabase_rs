use anyhow::{Result, Error};
use serde_json::Value;


pub fn parse_outer(query: &Value) -> bool {
    if let Some(query_str) = query.as_str() {
        let query_str = query_str.trim();
        
        let start_1: &str = r#"{"#;
        let end_1: &str = "}";

        query_str.starts_with(start_1) && query_str.ends_with(end_1)
    } else {
        true
    }
}


pub fn get_table_name(query: &Value) -> Result<String, Error> {
    if parse_outer(query) {
        let table_name: String = query["query"].as_str().unwrap_or("").to_string();

        // remove the brackets
        let table_name: String = table_name.replace("{", "").replace("}", "");

        // first word is table name
        let table_name: String = table_name.split_whitespace().next().unwrap().to_string();

        // break the word off as soon as its not alphanumeric
        let table_name: String = table_name.chars().take_while(|c| c.is_alphanumeric()).collect();


        Ok(table_name)
    } else {
        Err(Error::msg("Invalid outer structure"))
    }
}