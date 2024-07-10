use serde_json::json;
use crate::SupabaseClient;
use crate::tests::methods::init::init;

pub async fn insert_if_unique_string() {
    /// Performs an insert_if_unique operation in an isolated scope.
    async fn insert_if_unique_inner(supabase_client: SupabaseClient) -> Result<(), String> {
        // Usage example
        let random_string: String = rand::random::<u64>().to_string();

        let response_inner: Result<String, String> = supabase_client
            .insert_if_unique(
                "test",
                json!({
                    "dog": random_string,
                }),
            )
            .await;

        match response_inner {
            Ok(response_inner) => Ok(()),
            Err(error) => {
                println!("Error: {:?}", error);
                Err(error)
            }
        }
    }

    let supabase_client: SupabaseClient = match init().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!(
                "\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m",
                e
            );
            return;
        }
    };
    let response: Result<(), String> = insert_if_unique_inner(supabase_client).await;

    assert!(response.is_ok());
}