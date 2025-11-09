//! # Select Operations and Query Building
//!
//! This module provides the core querying functionality for retrieving data from Supabase tables.
//! It implements a fluent query builder pattern that allows for intuitive, chainable operations
//! with comprehensive filtering, sorting, and pagination capabilities.
//!
//! ## 🎯 Core Concepts
//!
//! ### Query Builder Pattern
//! The select operations use a fluent API that allows you to chain multiple operations:
//! ```text
//! client.select("table") -> .eq("column", "value") -> .limit(10) -> .execute()
//! ```
//!
//! ### Performance Considerations
//! - **Column Selection**: Use `.columns()` to fetch only needed fields
//! - **Pagination**: Prefer `.range()` over `.offset()` for better performance
//! - **Filtering**: Apply filters early to reduce data transfer
//! - **Counting**: Use `.count()` sparingly as it's expensive on large tables
//!
//! ## 🔍 Available Filter Operations
//!
//! | Operator | Method | Description | Example |
//! |----------|--------|-------------|---------|
//! | `=` | `eq(column, value)` | Equal to | `.eq("status", "active")` |
//! | `!=` | `neq(column, value)` | Not equal to | `.neq("deleted", "true")` |
//! | `>` | `gt(column, value)` | Greater than | `.gt("age", "18")` |
//! | `<` | `lt(column, value)` | Less than | `.lt("score", "100")` |
//! | `>=` | `gte(column, value)` | Greater than or equal | `.gte("created_at", "2024-01-01")` |
//! | `<=` | `lte(column, value)` | Less than or equal | `.lte("price", "50.00")` |
//! | `IN` | `in_(column, values)` | Value in list | `.in_("category", &["tech", "science"])` |
//! | `FTS` | `text_search(column, query)` | Full-text search | `.text_search("content", "rust")` |
//!
//! ## 📄 Pagination Methods
//!
//! | Method | Description | Performance | Use Case |
//! |--------|-------------|-------------|----------|
//! | `range(from, to)` | PostgREST range header | ✅ Fast | Recommended for pagination |
//! | `limit(n)` | Limit number of results | ✅ Fast | Simple result limiting |
//! | `offset(n)` | Skip n records | ⚠️ Slower | Use sparingly, prefer range |
//! | `count()` | Count matching records | ❌ Expensive | Use only when necessary |
//!
//! ## 📖 Usage Examples
//!
//! ### Basic Querying
//!
//! ```rust,no_run
//! use supabase_rs::SupabaseClient;
//! use serde_json::Value;
//!
//! # async fn example() -> Result<(), String> {
//! # let client = SupabaseClient::new("url".to_string(), "key".to_string()).unwrap();
//! // Simple select with filtering
//! let users: Vec<Value> = client
//!     .select("users")
//!     .eq("status", "active")
//!     .execute()
//!     .await?;
//!
//! println!("Found {} active users", users.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Advanced Filtering
//!
//! ```rust,no_run
//! # use supabase_rs::SupabaseClient;
//! # use serde_json::Value;
//! # async fn example() -> Result<(), String> {
//! # let client = SupabaseClient::new("url".to_string(), "key".to_string()).unwrap();
//! // Complex filtering with multiple conditions
//! let filtered_products: Vec<Value> = client
//!     .select("products")
//!     .gte("price", "10.00")              // Price >= $10
//!     .lte("price", "100.00")             // Price <= $100
//!     .neq("category", "discontinued")     // Not discontinued
//!     .in_("brand", &["apple", "samsung", "google"])  // Specific brands
//!     .text_search("description", "smartphone")        // Full-text search
//!     .order("price", true)               // Sort by price ascending
//!     .limit(50)                          // Limit results
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Column Selection and Pagination
//!
//! ```rust,no_run
//! # use supabase_rs::SupabaseClient;
//! # use serde_json::Value;
//! # async fn example() -> Result<(), String> {
//! # let client = SupabaseClient::new("url".to_string(), "key".to_string()).unwrap();
//! // Select specific columns for efficiency
//! let user_profiles: Vec<Value> = client
//!     .from("users")
//!     .columns(vec!["id", "name", "email", "avatar_url"])
//!     .eq("verified", "true")
//!     .range(0, 24)                       // Get first 25 records (0-24 inclusive)
//!     .order("created_at", false)         // Newest first
//!     .execute()
//!     .await?;
//!
//! // Offset-based pagination (less efficient but sometimes needed)
//! let page_2: Vec<Value> = client
//!     .from("posts")
//!     .columns(vec!["id", "title", "excerpt"])
//!     .eq("published", "true")
//!     .limit(10)
//!     .offset(10)                         // Skip first 10 records
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Count Operations
//!
//! > **⚠️ Performance Warning**: Count operations can be expensive on large tables. Use judiciously.
//!
//! ```rust,no_run
//! # use supabase_rs::SupabaseClient;
//! # use serde_json::Value;
//! # async fn example() -> Result<(), String> {
//! # let client = SupabaseClient::new("url".to_string(), "key".to_string()).unwrap();
//! // Count with filters (recommended)
//! let active_user_count: Vec<Value> = client
//!     .select("users")
//!     .eq("status", "active")
//!     .count()
//!     .execute()
//!     .await?;
//!
//! // Count all records (expensive on large tables)
//! let total_users: Vec<Value> = client
//!     .select("users")
//!     .count()
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## ⚡ Performance Tips
//!
//! 1. **Use Column Selection**: Only fetch columns you need
//! 2. **Apply Filters Early**: Reduce data transfer with specific filters
//! 3. **Prefer Range Over Offset**: Range-based pagination is more efficient
//! 4. **Limit Results**: Always use reasonable limits to prevent large responses
//! 5. **Index Your Filters**: Ensure filtered columns are indexed in your database
//!
//! ## 🔧 Error Handling
//!
//! All select operations return `Result<Vec<Value>, String>` for consistent error handling:
//!
//! ```rust,no_run
//! # use supabase_rs::SupabaseClient;
//! # use serde_json::Value;
//! # async fn example() -> Result<(), String> {
//! # let client = SupabaseClient::new("url".to_string(), "key".to_string()).unwrap();
//! match client.select("users").eq("id", "123").execute().await {
//!     Ok(users) => {
//!         if users.is_empty() {
//!             println!("No users found");
//!         } else {
//!             println!("Found {} users", users.len());
//!         }
//!     },
//!     Err(error) => {
//!         eprintln!("Query failed: {}", error);
//!         // Handle specific error cases
//!         if error.contains("401") {
//!             eprintln!("Authentication failed");
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use crate::query::{Query, QueryBuilder};
use crate::request::headers::HeadersTypes;
use crate::request::Headers;
use crate::success::handle_response;
use crate::SupabaseClient;

use reqwest::header::HeaderMap;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Response;
use serde_json::Value;

impl SupabaseClient {
    /// Initializes a `QueryBuilder` for a specified table.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table to be queried.
    ///
    /// # Returns
    /// A `QueryBuilder` instance configured for the specified table.
    // #[deprecate_until(remove = ">= 0.4.4", note = "`.select()` will be deprecated. Use `.from()` to specify the table name and then use `.select()` to pass the query string. This change will align with the official Supabase documentation for other languages.")]
    // #[cfg(not(feature = "nightly"))]
    pub fn select(&self, table_name: &str) -> QueryBuilder {
        QueryBuilder::new(self.clone(), table_name)
    }

    /// Alias for `select` which is closer to the official Supabase API style.
    ///
    /// This returns a `QueryBuilder` pointed at the given table. You can then chain
    /// filters like `.eq`, `.lte`, ordering via `.order`, and finally call `.execute()`.
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use supabase_rs::SupabaseClient;
    /// # async fn run(client: SupabaseClient) -> Result<(), String> {
    /// let rows = client
    ///     .from("pets")
    ///     .eq("name", "scooby")
    ///     .limit(5)
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from(&self, table_name: &str) -> QueryBuilder {
        QueryBuilder::new(self.clone(), table_name)
    }

    /// Executes a query against a specified table with a given query string.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table to be queried.
    /// * `query_string` - A string slice that holds the query parameters.
    ///
    /// # Returns
    /// A `Result` which is either a vector of `Value` representing the records fetched from the database
    /// or a `String` error message in case of failure.
    ///
    /// # Errors
    /// This function will return an error if the HTTP request fails or if the server returns a non-success status code.
    pub async fn execute(
        &self,
        table_name: &str,
        query_string: &str,
    ) -> Result<Vec<Value>, String> {
        // Build the client and the endpoint
        let endpoint: String = self.endpoint(table_name);
        let endpoint: String = format!("{endpoint}?{query_string}");

        #[cfg(feature = "nightly")]
        println!("\x1b[33mEndpoint: {}\x1b[0m", endpoint);

        #[cfg(feature = "nightly")]
        use crate::nightly::print_nightly_warning;
        #[cfg(feature = "nightly")]
        print_nightly_warning();

        let endpoint: String = if endpoint.ends_with("?count=exact") {
            endpoint.replace("?count=exact", "")
        } else {
            endpoint
        };

        // create headers with default values
        let mut headers: Headers = Headers::with_defaults(&self.api_key, &self.api_key);

        headers.insert(HeadersTypes::AcceptProfile.as_str(), self.schema.as_str());

        // convert headers to HeaderMap
        let mut header_map: HeaderMap = HeaderMap::new();
        for (key, value) in headers.get_headers() {
            header_map.insert(
                HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?,
                HeaderValue::from_str(&value).map_err(|e| e.to_string())?,
            );
        }

        // send the request
        let response: Response = match self.client.get(&endpoint).headers(header_map).send().await {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        // process the response
        handle_response(response).await
    }

    /// Executes a query against a specified table with a Query object that can contain range information.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table to be queried.
    /// * `query` - A Query object containing parameters, filters, sorts, and optional range.
    ///
    /// # Returns
    /// A `Result` which is either a vector of `Value` representing the records fetched from the database
    /// or a `String` error message in case of failure.
    ///
    /// # Errors
    /// This function will return an error if the HTTP request fails or if the server returns a non-success status code.
    pub async fn execute_with_query(
        &self,
        table_name: &str,
        query: &Query,
    ) -> Result<Vec<Value>, String> {
        // Build the client and the endpoint
        let endpoint: String = self.endpoint(table_name);
        let query_string = query.build();
        let endpoint: String = format!("{endpoint}?{query_string}");

        let endpoint: String = if endpoint.ends_with("?count=exact") {
            endpoint.replace("?count=exact", "")
        } else {
            endpoint
        };

        // create headers with default values
        let mut headers: Headers = Headers::with_defaults(&self.api_key, &self.api_key);

        // Add Range header if range is set
        if let Some((from, to)) = query.get_range() {
            headers.insert("Range", &format!("{}-{}", from, to));
        }

        headers.insert(HeadersTypes::AcceptProfile.as_str(), self.schema.as_str());

        // convert headers to HeaderMap
        let mut header_map: HeaderMap = HeaderMap::new();
        for (key, value) in headers.get_headers() {
            header_map.insert(
                HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?,
                HeaderValue::from_str(&value).map_err(|e| e.to_string())?,
            );
        }

        // send the request
        let response: Response = match self.client.get(&endpoint).headers(header_map).send().await {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        // process the response
        handle_response(response).await
    }
}
