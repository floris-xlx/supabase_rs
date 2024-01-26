use supabase_rs::SupabaseClient;

use dotenv::dotenv;
use std::env::var;
use serde_json::json;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let supabase_client: SupabaseClient = initialize_supabase_client().await;
    let response = update(supabase_client).await;

    println!("Response: {:?}", response);
}


async fn initialize_supabase_client() -> SupabaseClient {
    dotenv().ok();
    let supabase_client: SupabaseClient = SupabaseClient::new(
        var("SUPABASE_URL").unwrap(),
        var("SUPABASE_KEY").unwrap()
    );

    supabase_client
}

// update beta table set email_address = 'test' where id = '1
async fn update(
    supabase_client: SupabaseClient
) {

    let response: Result<(), String> = supabase_client
        .update(
            "beta",
            "1",
            json!({
                "email_address": "test"
            })
        )
        .await;

    match response {
        Ok(_) => println!("Update successful!"),
        Err(error) => println!("Error: {}", error)
    }
}