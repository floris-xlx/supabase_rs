//! This module contains the `select()` function
//!
//! ## Usage
//! First make sure you have initialized the Supabase Client
//! [Initalizing the SupabaseClient](#lib)
//!
//! This will return all `dog` rows where the value is `scooby` in the `animals` table
//! ```rust
//! use supabase_rs::SupabaseClient;
//! use dotenv::dotenv;
//! use std::env::var;
//! use serde_json::Value;
//!
//! async fn select_scooby(
//!      supabase_client: SupabaseClient
//! )-> Result<(), String>{
//!     let data: Result<Vec<Value>, String> = supabase_client
//!         .select("animals")
//!         .eq("dog", "scooby")
//!         .execute()
//!     .await;
//!
//! match data {
//!    Ok(data) => {
//!        println!("Data: {:?}", data);
//!        Ok(())
//!    },
//!    Err(error) => {
//!        println!("Error: {:?}", error);
//!        Err(error)
//!    }
//!}
//! ```
//!
//! ## Methods / Operators
//!
//! ### eq
//! This method checks if the Column is equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .eq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//! ### neq
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .neq("dog", "scooby")
//!     .execute()
//!     .await;
//! ```
//! ### gt
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .gt("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### lt
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .lt("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### gte
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .gte("weight", "100")
//!     .execute()
//!     .await;
//! ```
//! ### lte
//! This method checks if the Column is not equal to a value
//! ```rust
//! let data: Result<Vec<Value>, String> = supabase_client
//!     .select("animals")
//!     .lte("weight", "100")
//!     .execute()
//!     .await;
//! ```

use reqwest;
use serde_json::Value;
use reqwest::Client;
use reqwest::Response;
use std::collections::HashMap;

use crate::SupabaseClient;

// here we're gonna construct the request for the update
// building the Query builder

pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals
}


pub enum SortOrder {
    Ascending,
    Descending
}


pub struct Filter {
    column: String,
    operator: Operator,
    value: String
}


pub struct Sort {
    _column: String,
    _order: SortOrder
}


pub struct Query {
    params: HashMap<String, String>,
}

pub struct QueryBuilder {
    client: SupabaseClient,
    query: Query,
    table_name: String,
}

impl QueryBuilder {
    pub fn new(client: SupabaseClient, table_name: &str) -> Self {
        QueryBuilder {
            client,
            query: Query::new(),
            table_name: table_name.to_string(),
        }
    }

    /// This method checks if the Column is equal to a value
    pub fn eq(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("eq.{}", value));
        self
    }

    /// This method checks if the Column is not equal to a value
    pub fn neq(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("neq.{}", value));
        self
    }

    /// This method checks if the Column is greater than a value
    pub fn gt(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("gt.{}", value));
        self
    }

    /// This method checks if the Column is less than a value
    pub fn lt(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("lt.{}", value));
        self
    }

    /// This method checks if the Column is greater than or equal to a value
    pub fn gte(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("gte.{}", value));
        self
    }

    /// This method checks if the Column is less than or equal to a value
    pub fn lte(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("lte.{}", value));
        self
    }

    /// This is a mandatory method to execute the select query
    pub async fn execute(self) -> Result<Vec<Value>, String> {
        self.client.execute(&self.table_name, self.query.build().as_str()).await
    }
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
        self.params.insert(key.to_string(), value.to_string());
    }

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
    pub fn select(&self, table_name: &str) -> QueryBuilder {
        QueryBuilder::new(self.clone(), table_name)
    }

    // Updated execute function
    async fn execute(
        &self,
        table_name: &str,
        query_string: &str,
    ) -> Result<Vec<Value>, String> {
        // Build the client and the endpoint
        let endpoint: String = format!("{}/rest/v1/{}?{}", self.url, table_name, query_string);
        let client: Client = Client::new();

        // Send the request
        let response: Response = match client
            .get(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        // Process the response
        if response.status().is_success() {
            match response.json::<Vec<Value>>().await {
                Ok(records) => Ok(records),
                Err(error) => Err(error.to_string()),
            }
        } else {
            Err(response.status().to_string())
        }
    }
}