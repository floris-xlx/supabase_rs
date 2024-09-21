use crate::tests::methods::init::init;
use crate::SupabaseClient;
use serde_json::{json, Value};

pub async fn select() {
    /// Performs a select operation in an isolated scope.
    async fn select_inner(supabase_client: SupabaseClient) -> Result<(), String> {
        // Usage example

        let response_inner: Result<Vec<Value>, String> =
            supabase_client.select("test").execute().await;

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
    let response: Result<(), String> = select_inner(supabase_client).await;

    assert!(response.is_ok());
}
