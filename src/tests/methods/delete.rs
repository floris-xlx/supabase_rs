
use serde_json::{json, Value};
use crate::SupabaseClient;
use crate::tests::methods::init::init;


pub async fn delete() {
    /// Performs a select_filter operation in an isolated scope.
    async fn delete_inner(supabase_client: SupabaseClient) -> Result<(), String> {
        // Usage example

        let response_inner: Result<(), String> =
            supabase_client.delete("test", "1476105020679346924").await;

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
    let response: Result<(), String> = delete_inner(supabase_client).await;

    assert!(response.is_ok());
}