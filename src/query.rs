//! This module contains the `QueryBuilder` struct and its associated methods for building and executing SQL queries.
//!
//! This module provides the functionality to construct and manipulate SQL queries for the Supabase client.
//! It includes definitions for various query components such as filters, sorting, and the main `Query` structure.
//!
//! # Features
//! - Building complex SQL queries with ease.
//! - Support for multiple types of filters and sorting orders.
//! - Integration with the SupabaseClient for executing queries.
//!
//! # Examples
//! Basic usage:
//! ```
//! use supabase_rs::query::{Query, Filter, Operator, Sort, SortOrder};
//!
//! let mut query = Query::new();
//! let filter = Filter {
//!     column: "age".to_string(),
//!     operator: Operator::GreaterThan,
//!     value: "30".to_string(),
//! };
//! let sort = Sort {
//!     _column: "name".to_string(),
//!     _order: SortOrder::Ascending,
//! };
//! query.add_filter(filter);
//! query.add_sort(sort);
//! let query_string = query.build();
//! ```

use crate::SupabaseClient;
use serde_json::Value;
use std::{collections::HashMap, path::Display};

/// Represents the type of comparison to be performed in a query filter.
pub enum Operator {
    /// Represents equality comparison.
    Equals,
    /// Represents inequality comparison.
    NotEquals,
    /// Represents a comparison to check if a value is greater than another.
    GreaterThan,
    /// Represents a comparison to check if a value is less than another.
    LessThan,
    /// Represents a comparison to check if a value is greater than or equal to another.
    GreaterThanOrEquals,
    /// Represents a comparison to check if a value is less than or equal to another.
    LessThanOrEquals,
}

/// Specifies the order in which results should be sorted.
pub enum SortOrder {
    /// Results should be sorted in ascending order.
    Ascending,
    /// Results should be sorted in descending order.
    Descending,
}

/// Represents a filter to be applied to a query, consisting of a column name, an operator, and a value to compare against.
pub struct Filter {
    /// The name of the column to which the filter applies.
    column: String,
    /// The operator that defines the type of comparison to be performed.
    operator: Operator,
    /// The value to compare against the column's values.
    value: String,
}

/// Represents sorting criteria for query results, consisting of a column name and the order of sorting.
pub struct Sort {
    /// The name of the column by which to sort.
    _column: String,
    /// The order in which to sort the results.
    _order: SortOrder,
}

/// Represents a query with a collection of parameters that define specific conditions and sorting orders.
#[derive(Debug)]
pub struct Query {
    /// A map where each key-value pair represents a column and the condition or sorting order applied to it.
    params: HashMap<String, String>,
}

/// A `QueryBuilder` is used to construct and manage SQL queries for a specific table using a `SupabaseClient`.
///
/// It holds the state of the query being built, including the target table and any parameters or filters that have been set.
///
/// # Fields
/// - `client`: The `SupabaseClient` used to execute the query.
/// - `query`: A `Query` object that stores the parameters and conditions of the SQL query.
/// - `table_name`: The name of the table in the database to which the query will be applied.
#[derive(Debug)]
pub struct QueryBuilder {
    client: SupabaseClient,
    query: Query,
    table_name: String, // option columns
}

impl QueryBuilder {
    /// Constructs a new `QueryBuilder` for a specified table.
    ///
    /// # Arguments
    /// * `client` - A `SupabaseClient` instance to be used for the query.
    /// * `table_name` - A string slice that specifies the table name for the query.
    ///
    /// # Returns
    /// Returns a new instance of `QueryBuilder`.
    pub fn new(client: SupabaseClient, table_name: &str) -> Self {
        QueryBuilder {
            client,
            query: Query::new(),
            table_name: table_name.to_string(),
        }
    }

    pub fn columns(mut self, columns: Vec<&str>) -> QueryBuilder {
        // add query params &select=column1,column2
        let columns_str = columns.join(",");
        self.query.add_param("select", &columns_str);
        self
    }

    /// Adds a filter to the query to check if the column is equal to a specified value.
    ///
    /// # Arguments
    /// * `column` - The column name to apply the filter.
    /// * `value` - The value to compare against the column.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn eq(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("eq.{}", value));
        self
    }

    /// Adds a filter to the query to check if the column is not equal to a specified value.
    ///
    /// # Arguments
    /// * `column` - The column name to apply the filter.
    /// * `value` - The value to compare against the column.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn neq(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("neq.{}", value));
        self
    }

    /// Adds a filter to the query to check if the column is greater than a specified value.
    ///
    /// # Arguments
    /// * `column` - The column name to apply the filter.
    /// * `value` - The value to compare against the column.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn gt(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("gt.{}", value));
        self
    }

    /// Adds a filter to the query to check if the column is less than a specified value.
    ///
    /// # Arguments
    /// * `column` - The column name to apply the filter.
    /// * `value` - The value to compare against the column.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn lt(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("lt.{}", value));
        self
    }

    /// Adds a filter to the query to check if the column is greater than or equal to a specified value.
    ///
    /// # Arguments
    /// * `column` - The column name to apply the filter.
    /// * `value` - The value to compare against the column.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn gte(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("gte.{}", value));
        self
    }

    /// Adds a filter to the query to check if the column is less than or equal to a specified value.
    ///
    /// # Arguments
    /// * `column` - The column name to apply the filter.
    /// * `value` - The value to compare against the column.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn lte(mut self, column: &str, value: &str) -> Self {
        self.query.add_param(column, &format!("lte.{}", value));
        self
    }

    /// Adds a parameter to the query to count the exact number of rows that match the query.
    ///
    /// # Returns
    /// Returns the `QueryBuilder` instance to allow for method chaining.
    pub fn count(mut self) -> Self {
        self.query.add_param("count", "exact");
        self
    }

    /// Executes the constructed query against the database.
    ///
    /// # Returns
    /// Returns a `Result` containing either a vector of `Value` representing the fetched records, or a `String` error message.
    pub async fn execute(self) -> Result<Vec<Value>, String> {
        self.client
            .execute(&self.table_name, self.query.build().as_str())
            .await
    }
}

impl Filter {
    /// Constructs a new `Filter` instance.
    ///
    /// # Arguments
    /// * `column` - A `String` specifying the column name to which the filter will apply.
    /// * `operator` - An `Operator` enum specifying the type of comparison (e.g., Equals, NotEquals).
    /// * `value` - A `String` representing the value to compare against the column.
    ///
    /// # Returns
    /// Returns a new `Filter` instance containing the specified column, operator, and value.
    ///
    /// # Examples
    /// ```
    /// # use supabase_rs::query::{Filter, Operator};
    /// let filter = Filter::new("age".to_string(), Operator::GreaterThan, "30".to_string());
    /// ```
    pub fn new(column: String, operator: Operator, value: String) -> Filter {
        Filter {
            column,
            operator,
            value,
        }
    }
}

impl std::fmt::Display for Filter {
    /// Converts the filter into a query string format.
    ///
    /// This method formats the filter's column, operator, and value into a string
    /// that can be used directly in a query URL. The operator is converted to its
    /// corresponding string representation (`eq`, `neq`, `gt`, `lt`, `gte`, `lte`).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use supabase_rs::query::{Filter, Operator};
    /// let filter = Filter::new("age".to_string(), Operator::GreaterThan, "30".to_string());
    /// assert_eq!(filter.to_string(), "age.gt=30");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}={}",
            self.column,
            match self.operator {
                Operator::Equals => "eq",
                Operator::NotEquals => "neq",
                Operator::GreaterThan => "gt",
                Operator::LessThan => "lt",
                Operator::GreaterThanOrEquals => "gte",
                Operator::LessThanOrEquals => "lte",
            },
            self.value
        )
    }
}

// implementation of the query builder
impl Default for Query {
    fn default() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
}

impl Query {
    /// Constructs a new `Query` instance using the default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// let query = Query::new();
    /// ```
    pub fn new() -> Query {
        Query::default()
    }

    /// Adds a key-value pair to the query parameters.
    ///
    /// # Arguments
    /// * `key` - A string slice that holds the key of the parameter.
    /// * `value` - A string slice that holds the value of the parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use supabase_rs::query::Query;
    /// let mut query = Query::new();
    /// query.add_param("name", "John Doe");
    /// ```
    pub fn add_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    /// Builds and returns the query string from the current state of the query parameters.
    ///
    /// # Returns
    /// A `String` that represents the URL-encoded query string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use supabase_rs::query::Query;
    /// let mut query = Query::new();
    /// query.add_param("name", "John Doe");
    /// query.add_param("age", "30");
    /// let query_string = query.build();
    /// assert_eq!(query_string, "name=John Doe&age=30&");
    /// ```
    pub fn build(&self) -> String {
        self.params
            .iter()
            .map(|(key, value)| format!("{}={}&", key, value))
            .collect::<Vec<String>>()
            .join("")
    }
}
