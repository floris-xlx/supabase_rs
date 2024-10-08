use crate::tests::methods::init::init;
use crate::SupabaseClient;
use serde_json::json;

pub async fn upsert_numeric() {
    /// Performs a select_filter operation in an isolated scope.
    async fn upsert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
        // Usage example

        let id: String = "8826759220049045588".to_string();
        let email: i64 = 1234;

        // Usage example
        let response_inner = supabase_client
            .upsert(
                "test",
                &id,
                json!({
                    "email": email
                }),
            )
            .await;

        match response_inner {
            Ok(_) => Ok(()),

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
    let response: Result<(), String> = upsert_inner(supabase_client).await;

    assert!(response.is_ok());
}
