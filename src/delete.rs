use crate::SupabaseClient;
use serde_json::{
    json,
    Value
};
use reqwest::{
    Client,
    Response
};


impl SupabaseClient {
    /// Deletes a row in the table
    pub async fn delete(
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
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

}