use serde_json::{
    json,
    Value
};
use reqwest::{
    Client,
    Response
};

use crate::SupabaseClient;


impl SupabaseClient {
    /// Updates a row in the table, based on the id
    pub async fn update(
        &self,
        table_name: &str,
        id: &str,
        body: Value
    ) -> Result<(), String> {

        // endpoint and client construction
        let endpoint: String = format!(
            "{}/rest/v1/{}?id=eq.{}",
            self.url, table_name, id
        );
        let client: Client = Client::new();

        let response: Response = match client
            .patch(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(error) => return Err(error.to_string())
            };

        if response.status().is_success() {
            return Ok(())
        } else {
            return Err(response.status().to_string())
        }
    }


    /// Creates a row in the table, or updates if the id already exists
    pub async fn upsert(
        &self,
        table_name: &str,
        id: &str,
        mut body: Value
    ) -> Result<(), String> {

        // endpoint and client construction
        let endpoint: String = format!("{}/rest/v1/{}", self.url, table_name);
        let client: Client = Client::new();

        body["id"] = json!(id);

        let response: Response = match client
            .post(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", &format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=merge=duplicates")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(error) => return Err(error.to_string())
            };

        if response.status().is_success() {
            return Ok(())
        } else {
            return Err(response.status().to_string())
        }
    }
}