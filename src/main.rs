use supabase_rs::SupabaseClient;

use dotenv::dotenv;
use std::env::var;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let supabase_client: SupabaseClient = initialize_supabase_client().await;
    let response = select_test(supabase_client).await;

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
async fn select_test(
    supabase_client: SupabaseClient
)-> Result<(), String>{

    let response = supabase_client
    .select(
        "beta",
        "email_address",
        "eq.test"
    ).await;


    match response {
        Ok(response) => {
            println!("Response: {:?}", response);
            Ok(())
        },
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}