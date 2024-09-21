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

// graphql testing
// use supabase_rs::graphql::utils::format_endpoint::endpoint;
// use supabase_rs::graphql::utils::headers::{ self, headers };
// use supabase_rs::graphql::request::Request;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv::dotenv().ok();

    let supabase_client: SupabaseClient = SupabaseClient::new(
        std::env::var("SUPABASE_URL").unwrap(),
        std::env::var("SUPABASE_KEY").unwrap(),
    );

    let user_id = "xx-xx-xx-xx-xx";

    // let request_graphql: Request = Request::new(
    //     supabase_client,
    //     json!({
    //         "query": format!(
    //             r#"
    //             {{
    //                 usersCollection(filter: {{user_id: {{eq: \"{}\"}}}}) {{
    //                     edges {{
    //                         node {{
    //                             user_id,
    //                             username,
    //                             email
    //                         }}
    //                     }}
    //                 }}
    //             }}
    //             "#,
    //             user_id
    //         ),
    //     }),
    //     supabase_rs::graphql::RootTypes::Query
    // );
    // let response: Result<serde_json::Value, anyhow::Error> = request_graphql.send().await;

    // match response {
    //     Ok(response) => println!("{:#?}", response),
    //     Err(error) => println!("{:#?}", error),
    // }
}
