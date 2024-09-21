use crate::tests::methods::init::init;
use crate::SupabaseClient;
use serde_json::{json, Value};

pub async fn select_stacked_queries() {
    /// Performs a select_filter operation in an isolated scope.
    async fn upsert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
        // Usage example

        let id: String = "8826759220049045588".to_string();
        let email: String = "floris@xylex.ai".to_string();

        // Usage example
        let response_inner = supabase_client
            .select("test")
            .gt("number", "10")
            .gt("number", "20")
            .execute()
            .await;

        match response_inner {
            Ok(response_inner) => Ok(()),

            Err(error) => {
                eprintln!("\x1b[31mError: {:?}\x1b[0m", error);
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
