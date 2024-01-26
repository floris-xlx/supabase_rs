use reqwest;
use serde_json::Value;
use reqwest::Client;
use reqwest::Response;
use std::collections::HashMap;

use crate::SupabaseClient;

// here we're gonna construct the request for the update
// building the Query builder

#[derive(Debug)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals
}

#[derive(Debug)]
pub enum SortOrder {
    Ascending,
    Descending
}

#[derive(Debug)]
pub struct Filter {
    column: String,
    operator: Operator,
    value: String
}

#[derive(Debug)]
pub struct Sort {
    column: String,
    order: SortOrder
}

#[derive(Debug)]
pub struct Query {
    params: HashMap<String, String>,
}

impl Filter {
    // constructs a new filter
    pub fn new(
        column: String,
        operator: Operator,
        value: String
    ) -> Filter {
        Filter {
            column,
            operator,
            value
        }
    }

    // converts the filter to a string
    pub fn to_string(
        &self
    ) -> String {
        format!(
            "{}.{}={}",
            self.column,
            match self.operator {
                Operator::Equals => "eq",
                Operator::NotEquals => "neq",
                Operator::GreaterThan => "gt",
                Operator::LessThan => "lt",
                Operator::GreaterThanOrEquals => "gte",
                Operator::LessThanOrEquals => "lte"
            },
            self.value
        )
    }
}

// implementation of the query builder
impl Query {
    // Constructs a new query
    pub fn new() -> Query {
        Query {
            params: HashMap::new(),
        }
    }

    // Method to add a key-value pair to the query
    pub fn add_param(&mut self, key: &str, value: &str) {
        println!("Adding param: {}={}", key, value);
        self.params.insert(key.to_string(), value.to_string());
    }

    // method to add our operator


    // Method to build the query string
    pub fn build(&self) -> String {
        let mut query_string = String::new();
        for (key, value) in &self.params {
            query_string.push_str(&format!("{}={}&", key, value));
        }
        query_string
    }
}


// Modify your select function
impl SupabaseClient {
    /// Selects rows from the table, based on the search column and query
    pub async fn select(
        &self,
        table_name: &str,
        search_column: &str,
        search_query: &str,
    ) -> Result<Vec<Value>, String> {
        // Build the client and the endpoints
        let endpoint: String = format!("{}/rest/v1/{}?select=*", self.url, table_name);
        let client: Client = Client::new();

        let mut query = Query::new();
        query.add_param(search_column, search_query);

        let response: Response = match client
            .get(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .query(&query.params) // Use query.params here
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        if response.status().is_success() {
            let records: Result<Vec<Value>, _> = match response.json().await {
                Ok(records) => Ok(records),
                Err(error) => Err(error.to_string()),
            };
            return records;
        } else {
            return Err(response.status().to_string());
        }
    }
}