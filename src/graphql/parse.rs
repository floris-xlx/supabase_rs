use anyhow::{Result, Error};
use serde_json::Value;
use crate::graphql::error_types::{illegal_table_name, table_does_not_exist, field_does_not_exist_on_table, table_name_does_not_end_with_collection};


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
        let query_str: &str = query["query"].as_str().unwrap_or("");
        println!("Query: {}", query_str);
        // remove all the { } and then get the first alphanumeric word from the query
        let query_str: String = query_str.replace("{", "").replace("}", "");
        let query_str: String = query_str.trim().to_string();
        let query_str: Vec<&str> = query_str.split_whitespace().collect();
        let mut table_name: String = query_str[0].to_string();

        // remove all beyond the last alphanumeric char
        for (i, c) in table_name.chars().enumerate() {
            if !c.is_alphanumeric() {
                table_name = table_name[..i].to_string();
                break;
            }
        }

        // if the table name doesnt end with Collection, add it
        if !table_name.ends_with("Collection") {
            table_name_does_not_end_with_collection(&table_name);
        }

        Ok(table_name)
    } else {
        Err(Error::msg("Invalid outer structure"))
    }
}

   