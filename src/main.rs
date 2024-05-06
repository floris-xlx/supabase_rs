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
use serde_json::json;
use supabase_rs::SupabaseClient;



// update beta table set email_address = 'test' where id = '1
// async fn select_test(
//     supabase_client: SupabaseClient
// )-> Result<Vec<Value>, String>{

//     // Usage example
//     let response: Result<Vec<Value>, String> = supabase_client
//         .select("trades")
//         .gt("id", "1")
//         .count()
//         .execute()
//         .await;

//     match response {
//         Ok(response) => {
//             println!("Response: {:?}", response);
//             Ok(response)
//         },
//         Err(error) => {
//             println!("Error: {:?}", error);
//             Err(error)
//         }
//     }
// }



// async fn insert_test(supabase_client: SupabaseClient) -> Result<(), String> {
//     // Usage example
//     let response: Result<String, String> = supabase_client
//         .insert(
//             "stripe_customer_data",
//             json!({
//                 "email": "what da dog doing"

//             }),
//         )
//         .await;

//     match response {
//         Ok(response) => {
//             println!("Response: {:?}", response);
//             Ok(())
//         }
//         Err(error) => {
//             println!("Error: {:?}", error);
//             Err(error)
//         }
//     }
// }




async fn upsert_test(supabase_client: SupabaseClient) -> Result<(), String> {
    let email: String = "floris@xylex.ai".to_string();
    let id: String = "1".to_string();

    // Usage example
    let response= supabase_client
        .upsert(
            "stripe_customer_data",
            &id,
            json!({
                
                "email": email
            }),
        )
        .await;
    
    println!("Response: {:?}", response);
    Ok(())
}




#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv::dotenv().ok();

    let supabase_client: SupabaseClient = SupabaseClient::new(
        std::env::var("SUPABASE_URL").unwrap(),
        std::env::var("SUPABASE_KEY").unwrap(),
    ); 

    let response = upsert_test(supabase_client).await;



    println!("Response main: {:#?}", response);
}
