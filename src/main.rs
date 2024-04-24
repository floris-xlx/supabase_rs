// use supabase_rs::SupabaseClient;

// // use dotenv::dotenv;
// // use std::env::var;
// use serde_json::json;

// #[tokio::main]
// async fn main() {
//     let supabase_client: SupabaseClient = initialize_supabase_client().await;
//     let response = insert_test(supabase_client).await;

//     println!("Response: {:?}", response);
// }

// async fn initialize_supabase_client() -> SupabaseClient {
//     dotenv().ok();

//     let supabase_client: SupabaseClient = SupabaseClient::new(
//         var("SUPABASE_URL").unwrap(),
//         var("SUPABASE_KEY").unwrap()
//     );

//     supabase_client
// }

// update beta table set email_address = 'test' where id = '1
// async fn select_test(
//     supabase_client: SupabaseClient
// )-> Result<(), String>{

//     // Usage example
//     let response: Result<Vec<Value>, String> = supabase_client
//         .select("beta")
//         .gte("numba", "1000")
//         .execute()
//         .await;

//     match response {
//         Ok(response) => {
//             println!("Response: {:?}", response);
//             Ok(())
//         },
//         Err(error) => {
//             println!("Error: {:?}", error);
//             Err(error)
//         }
//     }
// }

use serde_json::json;
use supabase_rs::SupabaseClient;

async fn insert_test(supabase_client: SupabaseClient) -> Result<(), String> {
    // Usage example
    let response: Result<String, String> = supabase_client
        .insert(
            "test",
            json!({
                "dog": "what da dog doing"

            }),
        )
        .await;

    match response {
        Ok(response) => {
            println!("Response: {:?}", response);
            Ok(())
        }
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let supabase_client: SupabaseClient = SupabaseClient::new(
        "url".to_string(),
        "key".to_string(),
    );

    let response: Result<(), String> = insert_test(supabase_client).await;

    println!("Response: {:?}", response);
}
