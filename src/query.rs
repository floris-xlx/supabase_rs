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
//! ```ignore
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

// local imports
use crate::SupabaseClient;

// external imports
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Formatter;


/// Represents the type of comparison to be performed in a query filter.
#[derive(Debug)]
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
#[derive(Debug)]
pub enum SortOrder {
    /// Results should be sorted in ascending order.
    Ascending,
    /// Results should be sorted in descending order.
    Descending,
}

/// Represents a filter to be applied to a query, consisting of a column name, an operator, and a value to compare against.
#[derive(Debug)]
pub struct Filter {
    /// The name of the column to which the filter applies.
    pub column: String,
    /// The operator that defines the type of comparison to be performed.
    pub operator: Operator,
    /// The value to compare against the column's values.
    pub value: String,
}

/// Represents sorting criteria for query results, consisting of a column name and the order of sorting.
#[derive(Debug)]
pub struct Sort {
    /// The name of the column by which to sort.
    pub column: String,
    /// The order in which to sort the results.
    pub order: SortOrder,
}

/// Represents a query with a collection of parameters that define specific conditions and sorting orders.
#[derive(Debug, Default)]
pub struct Query {
    /// A map where each key-value pair represents a column and the condition or sorting order applied to it.
    pub params: Vec<(String, String)>,
    /// A vector of filters to be applied to the query results.
    pub filters: Vec<Filter>,
    /// A vector of sorting criteria to be applied to the query results.
    pub sorts: Vec<Sort>,
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
    pub client: SupabaseClient,
    pub query: Query,
    pub table_name: String, // option columns
}

