use crate::query::{QueryBuilder, Query, Filter, Sort};
use crate::SupabaseClient;

use serde_json::Value;

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


impl Query {
    /// Constructs a new `Query` instance using the default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// # use supabase_rs::query::Query;
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
        let key_value_pair = (key.to_string(), value.to_string());
        if !self.params.contains(&key_value_pair) {
            self.params.push(key_value_pair);
        }
    }

    /// Adds a filter to the query.
    ///
    /// # Arguments
    /// * `filter` - A `Filter` struct containing the column name, operator, and value for the filter.
    ///
    /// # Examples
    /// ```
    /// # use supabase_rs::query::{Query, Filter, Operator};
    /// let mut query = Query::new();
    /// let filter = Filter {
    ///     column: "age".to_string(),
    ///     operator: Operator::GreaterThan,
    ///     value: "30".to_string(),
    /// };
    /// query.add_filter(filter);
    /// ```
    pub fn add_filter(&mut self, filter: Filter) {
        self.filters.push(filter);
    }

    /// Adds a sorting criterion to the query.
    ///
    /// # Arguments
    /// * `sort` - A `Sort` struct containing the column name and the sorting order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use supabase_rs::query::{Query, Sort, SortOrder};
    /// let mut query = Query::new();
    /// let sort = Sort {
    ///     column: "name".to_string(),
    ///     order: SortOrder::Ascending,
    /// };
    /// query.add_sort(sort);
    /// ```
    pub fn add_sort(&mut self, sort: Sort) {
        self.sorts.push(sort);
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
        println!("{:?}", self.params);
        self.params
            .iter()
            .map(|(key, value)| format!("{}={}&", key, value))
            .collect::<Vec<String>>()
            .join("");

        
        let mut query_string: String = String::new();
        
        // add params
        query_string.push_str(
            self.params
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join("&")
                .as_str(),
        );
        if !self.filters.is_empty() {
            // add filters
            if !query_string.is_empty() {
                query_string.push('&');
            }
            query_string.push_str(
                self.filters
                    .iter()
                    .map(|filter| filter.to_string())
                    .collect::<Vec<String>>()
                    .join("&")
                    .as_str(),
            );
        }
        if !self.sorts.is_empty() {
            // add sorts
            if !query_string.is_empty() {
                query_string.push('&');
            }
            query_string.push_str(
                self.sorts
                    .iter()
                    .map(|sort| sort.to_string())
                    .collect::<Vec<String>>()
                    .join("&")
                    .as_str(),
            );
        }
        query_string
    }
}
