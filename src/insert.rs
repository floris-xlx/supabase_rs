use crate::{
    SupabaseClient,
    generate_random_id
};
use serde_json::{
    json,
    Value
};
use reqwest::{
    Client,
    Response
};


impl SupabaseClient {
    /// Inserts a row in the table
    pub async fn insert(
        &self,
        table_name: &str,
        mut body: Value
    ) -> Result<String, String> {

        let endpoint: String = format!("{}/rest/v1/{}", self.url, table_name);
        let client: Client = Client::new();
        let new_id: i64 = generate_random_id();
        body["id"] = json!(new_id);

        println!("Body: {:?}", body);
        // Make a GET request to the user endpoint
        let response: Response = match client
            .post(&endpoint)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await {
                Ok(response) => response,
                Err(e) => return Err(e.to_string())
            };

        if response.status().is_success() {
            Ok(new_id.to_string())

        } else if response.status().as_u16() == 409 {
            println!("\x1b[31mError 409: Duplicate entry. The value you're trying to insert may already exist in a column with a UNIQUE constraint.\x1b[0m");

            return Err("\x1b[31mError 409: Duplicate entry. The value you're trying to insert may already exist in a column with a UNIQUE constraint.\x1b[0m".to_string());
        } else {
            println!("Error: {:?}", response);
            return Err(response.status().to_string())
        }
    }


    /// Inserts a row in the table if the value is unique and does not exist in the table already
    pub async fn insert_if_unique(
        &self,
        table_name: &str,
        mut body: Value
    ) -> Result<String, String> {
        // get column name and value from body

        let column_name: String = body
            .as_object_mut()
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .to_string();

        // get the value keyed by the column name, which can be a number too
        let column_value = match body
            .as_object_mut()
            .unwrap()
            .get(&column_name)
            .unwrap() {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => panic!("Unsupported type for column value"),
            };

        // get the current value of the column and see if equal to the value in the body
        let response: Result<Vec<Value>, String> = self
            .select(table_name)
            .eq(&column_name, &column_value)
            .execute()
            .await;

        // if the response is empty, insert the row
        if response.unwrap().is_empty() {
            return self.insert(table_name, body).await;
        }

        // if the response is not empty, return an error
        Err("\x1b[31mError 409: Duplicate entry. The value you're trying to insert may already exist in a column with a UNIQUE constraint.\x1b[0m".to_string())
    }
}