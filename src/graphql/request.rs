use crate::SupabaseClient;

// local imports
use crate::graphql::Query;
use crate::graphql::RootTypes;
use crate::graphql::utils::format_endpoint::endpoint;
use crate::graphql::utils::headers::headers;
use crate::graphql::parse::{ parse_outer, get_table_name };

// custom errors
use crate::graphql::error_types::{
    illegal_table_name,
    table_does_not_exist,
    field_does_not_exist_on_table,
    illegal_field_name,
};

use serde_json::json;
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;

use anyhow::{ Result, Error as AnyError };

#[derive(Debug)]
pub struct Request {
    pub client: SupabaseClient,
    pub query: Value,
    pub root_type: RootTypes,
}

impl Request {
    pub fn new(client: SupabaseClient, query: Value, root_type: RootTypes) -> Self {
        Request {
            client,
            query,
            root_type,
        }
    }

    pub async fn format_query(&self) -> String {
        let query: String = match &self.root_type {
            RootTypes::Query =>
                format!(
                    r#"{{"query": "{}", "variables": {{}}}}"#,
                    self.query["query"].as_str().unwrap_or("")
                ),

            RootTypes::Mutation => format!("{}", self.query),

            // uncovered
            RootTypes::Subscription => format!("{}", self.query),
            RootTypes::Fragment => format!("{}", self.query),
        };

        // remove all the weird newlines and tabs
        let query: String = query.replace("\n", "").replace("\t", "");
        // spaces
        let query: String = query.replace(" ", "");

        query
    }

    pub async fn send(&self) -> Result<serde_json::Value, AnyError> {
        // verify query
        let query: Value = self.query.clone();

        // TEMP
        let table_name: String = get_table_name(&query).unwrap();
        let field_name: &str = "eads";

        println!("Table Name: {}", table_name);

        let headers_map: HashMap<String, String> = headers(&self.client);
        let endpoint_graphql: String = endpoint(&self.client);

        // format query

        let query: String = self.format_query().await;

        let client: Client = Client::new();
        let res: reqwest::Response = client
            .post(&endpoint_graphql)
            .header("apiKey", headers_map.get("apiKey").unwrap())
            .header("Content-Type", headers_map.get("Content-Type").unwrap())
            .body(query)
            .send().await?;

        let body: String = res.text().await.unwrap();
        let data: serde_json::Value = serde_json::from_str(&body).unwrap();

        println!("\x1b[1;34m{:#?}\x1b[0m", data);

        if data["errors"].is_array() {
            let message: Value = data["errors"][0]["message"].clone();
            let error_message: String = serde_json
                ::from_value(message)
                .unwrap_or_else(|_| "Failed to deserialize error message".to_string());
            let error_message: String = error_router(&error_message, field_name, &table_name).await;

            // return the errors key from the data
            let data: Value = data["errors"][0]["message"].to_string().parse().unwrap();
            return Err(AnyError::msg(data));
        } else {
            // if there are no errors
            let data: Value = data["data"].clone();
            return Ok(data);
        }
    }
}

pub async fn error_router(error_message: &str, field_name: &str, table_name: &str) -> String {
    let re: regex::Regex = regex::Regex::new(r#"Unknown field "[^"]*""#).unwrap();

    if re.is_match(&error_message) {
        return table_does_not_exist(table_name);
    } else if
        error_message
            .to_string()
            .contains("query parse error: Parse error at 1:2\nUnexpected `unsupported float")
    {
        return illegal_table_name(table_name);
    } else if
        error_message
            .to_string()
            .contains("query parse error: Parse error at 1:2\nUnexpected `unsupported integer")
    {
        return illegal_table_name(table_name);
    } else if
        regex::Regex
            ::new(r#"Unknown field '[^']*' on type '[^']*'"#)
            .unwrap()
            .is_match(&error_message)
    {
        return field_does_not_exist_on_table(field_name, table_name);
    } else if
        regex::Regex
            ::new(r#"query parse error: Parse error at \d+:\d+\nUnexpected `unsupported float"#)
            .unwrap()
            .is_match(&error_message)
    {
        return illegal_field_name(field_name);
    } else {
        return error_message.to_string();
    }
}
