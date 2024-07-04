#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(unused_parens)]
#![allow(unused_braces)]
#![allow(unused_macros)]

use serde_json::json;
use supabase_rs::SupabaseClient;
use serde_json::Value;
use dotenv::dotenv;
use std::env::var;
// graphql testing
// use supabase_rs::graphql::utils::format_endpoint::endpoint;
// use supabase_rs::graphql::utils::headers::{ self, headers };
// use supabase_rs::graphql::request::Request;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv::dotenv().ok();

    // let supabase_client: SupabaseClient = SupabaseClient::new(
    //     std::env::var("SUPABASE_URL").unwrap(),
    //     std::env::var("SUPABASE_KEY").unwrap()
    // );

    // let user_id = "xx-xx-xx-xx-xx";
    // /// Initializes the Supabase client by loading environment variables.
    // async fn init() -> Result<SupabaseClient, Box<dyn std::error::Error>> {
    //     dotenv().ok();

    //     let supabase_url = var("SUPABASE_URL")?;
    //     let supabase_key = var("SUPABASE_KEY")?;

    //     Ok(SupabaseClient::new(supabase_url, supabase_key))
    // }

    // async fn update_string() {
    //     /// Performs a select_filter operation in an isolated scope.
    //     async fn update_inner(supabase_client: SupabaseClient) -> Result<(), String> {
    //         // Usage example

    //         let id: String = "xlx-t-733e46a5eaa13ba30dc0f51bd9ef2bf8".to_string();


    //         let updated_body: Value =
    //             json!({
    //             "summary": "what da dog doing"
    //         });

    //         // Usage example
    //         let response_inner: Result<String, String> = supabase_client.update_with_column_name(
    //             "trades",
    //             "trade_hash",
    //             &id,
    //             updated_body
    //         ).await;

    //         match response_inner {
    //             Ok(response_inner) => { Ok(()) }

    //             Err(error) => {
    //                 println!("Error: {:?}", error);
    //                 Err(error)
    //             }
    //         }
    //     }

    //     let supabase_client: SupabaseClient = match init().await {
    //         Ok(client) => client,
    //         Err(e) => {
    //             eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
    //             return;
    //         }
    //     };
    //     let response: Result<(), String> = update_inner(supabase_client).await;

    //     assert_eq!(response.is_ok(), true);
    // }


    // update_string().await;
}
