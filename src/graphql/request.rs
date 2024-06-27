use crate::SupabaseClient;
use crate::graphql::parse::get_table_name;
use crate::graphql::utils::format_endpoint::endpoint;
use crate::graphql::utils::headers::headers;
use crate::graphql::{RootTypes, Query};
use crate::graphql::error_types::{
    field_does_not_exist_on_table, illegal_field_name, illegal_table_name, table_does_not_exist,
    failed_to_parse_json,
};

use regex::Regex;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use anyhow::{Error as AnyError, Result};

#[derive(Debug)]
pub struct Request {
    pub client: SupabaseClient,
    pub query: Value,
    pub root_type: RootTypes,
}

impl Request {
    pub fn new(client: SupabaseClient, query: Value, root_type: RootTypes) -> Self {
        Self {
            client,
            query,
            root_type,
        }
    }

    pub async fn format_query(&self) -> String {
        let query = match &self.root_type {
            RootTypes::Query => format!(
                r#"{{"query": "{}", "variables": {{}}}}"#,
                self.query["query"].as_str().unwrap_or("")
            ),
            _ => self.query.to_string(),
        };

        query.replace("\n", "").replace("\t", "").replace(" ", "")
    }

    pub async fn send(&self) -> Result<Value, AnyError> {
        let query = self.query.clone();
        let table_name = get_table_name(&query).map_err(|e| e)?;

        let headers_map = headers(&self.client);
        let endpoint_graphql = endpoint(&self.client);
        let formatted_query = self.format_query().await;

        // println!("formatted_query: {}", formatted_query);

        #[cfg(feature = "rustls")]
        let client = Client::builder().use_rustls_tls().build().unwrap();
        
        #[cfg(not(feature = "rustls"))]
        let client = Client::new();
        

        #[cfg(feature = "nightly")]
        use crate::nightly::print_nightly_warning;
        #[cfg(feature = "nightly")]
        print_nightly_warning();


        let res = client
            .post(&endpoint_graphql)
            .header("apiKey", headers_map.get("apiKey").unwrap())
            .header("Content-Type", headers_map.get("Content-Type").unwrap())
            .body(formatted_query)
            .send()
            .await?;

        let data: Value = res.json().await.map_err(|e| failed_to_parse_json(e.to_string()))?;

        // println!("{:#?}", data);

        if let Some(errors) = data["errors"].as_array() {
            let message = errors[0]["message"].clone();
            let error_message: String = serde_json::from_value(message)
                .unwrap_or_else(|_| "Failed to deserialize error message".to_string());
            let error_message = error_router(&error_message, "eads", &table_name).await;

            let parsed_data: Value = data["errors"][0]["message"]
                .to_string()
                .parse()
                .map_err(|e| AnyError::msg(format!("Failed to parse error message: {}", e)))?;

            return Err(AnyError::msg(parsed_data));
        }

        let data: Value = data["data"].clone();
 
        let data: Value = if data[&table_name].is_null() {
            data
        } else {
            data[table_name]["edges"].clone()
        };

        Ok(data)
    }
}

pub async fn error_router(error_message: &str, field_name: &str, table_name: &str) -> String {
    let re = match Regex::new(r#"Unknown field "[^"]*""#) {
        Ok(regex) => regex,
        Err(e) => return format!("Failed to create regex: {}", e),
    };
    
    if re.is_match(error_message) {
        return table_does_not_exist(table_name);
    } else if error_message.contains("query parse error: Parse error at 1:2\nUnexpected `unsupported float") {
        return illegal_table_name(table_name);
    } else if error_message.contains("query parse error: Parse error at 1:2\nUnexpected `unsupported integer") {
        return illegal_table_name(table_name);
    } else if Regex::new(r#"Unknown field '[^']*' on type '[^']*'"#)
        .unwrap()
        .is_match(error_message)
    {
        return field_does_not_exist_on_table(field_name, table_name);
    } else if Regex::new(r#"query parse error: Parse error at \d+:\d+\nUnexpected `unsupported float"#)
        .unwrap()
        .is_match(error_message)
    {
        return illegal_field_name(field_name);
    } else {
        return error_message.to_string();
    }
}
