//! This module provides the functionality to delete rows from a Supabase table.
//!
//! It leverages the Supabase REST API to send delete requests. The main functionality is encapsulated
//! in the `SupabaseClient` struct, which provides the `delete` method to perform the deletion.
//! 
//! ## Usage
//!     

use crate::SupabaseClient;
use serde_json::json;
use reqwest::{
    Client,
    Response
};


impl SupabaseClient {
    /// Deletes a row in the specified table based on the provided ID.
    ///
    /// # Arguments
    /// * `table_name` - A string slice that holds the name of the table from which to delete.
    /// * `id` - A string slice that holds the ID of the row to delete.
    /// * `body` - A JSON value containing the body of the request, typically specifying conditions for deletion.
    ///
    /// # Returns
    /// This method returns a `Result<(), String>`. On success, it returns `Ok(())`, and on failure, it returns
    /// `Err(String)` with an error message.
    ///
    /// # Examples
    /// ```
    /// use serde_json::json;
    /// use supabase_rs::SupabaseClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SupabaseClient::new("your_supabase_url", "your_supabase_key");
    ///     let result = client.delete("your_table_name", "row_id", json!({})).await;
    ///     match result {
    ///         Ok(_) => println!("Row deleted successfully"),
    ///         Err(e) => println!("Failed to delete row: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn delete(
        &self,
        table_name: &str,
        id: &str,
        //body: Value
    ) -> Result<(), String> {

        // Construct the endpoint URL for the delete operation
        let endpoint: String = format!(
            "{}/rest/v1/{}?id=eq.{}",
            self.url, table_name, id
        );
        
        #[cfg(feature = "rustls")]
        let client = Client::builder().use_rustls_tls().build().unwrap();
        
        #[cfg(not(feature = "rustls"))]
        let client = Client::new();
        

        #[cfg(feature = "nightly")]
        use crate::nightly::print_nightly_warning;
        #[cfg(feature = "nightly")]
        print_nightly_warning();


        let body: serde_json::Value = json!({}); // this is temporary, will be used for more complex queries

        // Send the delete request and handle the response
        let response: Response = match client
            .delete(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(error) => return Err(error.to_string())
            };


        // Check the HTTP status code of the response
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

}