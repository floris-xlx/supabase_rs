//! This module contains tests for the base module
//!
//! It includes tests for various CRUD operations using the `SupabaseClient`.
//! Each test is designed to interact with a Supabase database using the client's methods
//! such as `insert`, `insert_if_unique`, `select`, and `select_filter`.
//!
//! ## Tests
//!
//! - `insert`: Tests the insertion of a new row.
//! - `insert_if_unique`: Tests the insertion of a unique row.
//! - `select`: Tests the selection of rows.
//! - `select_filter`: Tests the selection of rows with a specific filter.
//! - `select_with_count`: Tests the selection of rows with a count.
//! - `select_with_count_and_filter`: Tests the selection of rows with a count and a filter.
//! - `delete`: Tests the deletion of a row.
//! - `upsert`: Tests the upsertion of a row.
//! - `update`: Tests the update of a row.
//!
//! These tests ensure that the basic functionalities of interacting with a Supabase database
//! are working as expected.
//! 

#[cfg(test)]
mod methods {
    use crate::SupabaseClient;
    use dotenv::dotenv;
    use serde_json::{Value, json};
    use std::env::var;

    /// Initializes the Supabase client by loading environment variables.
    async fn init() -> Result<SupabaseClient, Box<dyn std::error::Error>> {
        dotenv().ok();

        let supabase_url = var("SUPABASE_URL")?;
        let supabase_key = var("SUPABASE_KEY")?;

        Ok(SupabaseClient::new(supabase_url, supabase_key))
    }

    /// Tests the `insert` method of `SupabaseClient`.
    #[tokio::test]
    async fn insert() {

        /// Performs an insert operation in an isolated scope.
        async fn insert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            let response_inner: Result<String, String> = supabase_client
                .insert(
                    "test",
                    json!({
                        "dog": "what da dog doing"
                    }),
                )
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        
        let response: Result<(), String> = insert_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `insert` method of `SupabaseClient`.
    #[tokio::test]
    async fn insert_string() {

        /// Performs an insert operation in an isolated scope.
        async fn insert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            let response_inner: Result<String, String> = supabase_client
                .insert(
                    "test",
                    json!({
                        "dog": "what da dog doing2"
                    }),
                )
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = insert_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `insert` method of `SupabaseClient`.
    #[tokio::test]
    async fn insert_numeric() {

        /// Performs an insert operation in an isolated scope.
        async fn insert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            let response_inner: Result<String, String> = supabase_client
                .insert(
                    "test",
                    json!({
                        "dog": 1234
                    }),
                )
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = insert_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }


    /// Tests the `insert_if_unique` method of `SupabaseClient`.
    #[tokio::test]
    async fn insert_if_unique_string() {
        
        /// Performs an insert_if_unique operation in an isolated scope.
        async fn insert_if_unique_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            let random_string: String = rand::random::<u64>().to_string();
            
            let response_inner: Result<String, String> = supabase_client
                .insert_if_unique(
                    "test",
                    json!({
                        "dog": random_string,
                    }),
                )
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = insert_if_unique_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `insert_if_unique` method of `SupabaseClient`.
    #[tokio::test]
    async fn insert_if_unique_numeric() {
        
        /// Performs an insert_if_unique operation in an isolated scope.
        async fn insert_if_unique_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            let random_number: u64 = rand::random::<u64>();
            
            let response_inner: Result<String, String> = supabase_client
                .insert_if_unique(
                    "test",
                    json!({
                        "dog": random_number,
                    }),
                )
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = insert_if_unique_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }


    /// Tests the `select` method of `SupabaseClient`.
    #[tokio::test]
    async fn select() {
        
        /// Performs a select operation in an isolated scope.
        async fn select_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            
            let response_inner: Result<Vec<Value>, String> = supabase_client
                .select("test")
                .execute()
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = select_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `select_filter` method of `SupabaseClient`.
    #[tokio::test]
    async fn select_filter() {
        
        /// Performs a select_filter operation in an isolated scope.
        async fn select_filter_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            
            let response_inner: Result<Vec<Value>, String> = supabase_client
                .select("test")
                .eq("dog", "what da dog doing")
                .execute()
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = select_filter_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `select_filter` method of `SupabaseClient`.
    #[tokio::test]
    async fn select_with_count() {
        
        /// Performs a select_filter operation in an isolated scope.
        async fn select_with_count_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            
            let response_inner: Result<Vec<Value>, String> = supabase_client
                .select("test")
                .count()
                .execute()
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = select_with_count_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `select_filter` method of `SupabaseClient`.
    #[tokio::test]
    async fn select_with_count_and_filter() {
        
        /// Performs a select_filter operation in an isolated scope.
        async fn select_with_count_and_filter_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            
            let response_inner: Result<Vec<Value>, String> = supabase_client
                .select("test")
                .eq("dog", "what da dog doing")
                .count()
                .execute()
                .await;
            println!("{:?}", response_inner);
            
            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = select_with_count_and_filter_inner(supabase_client).await;
        println!("{:?}", response);

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `select_filter` method of `SupabaseClient`.
    #[tokio::test]
    async fn delete() {
        
        /// Performs a select_filter operation in an isolated scope.
        async fn delete_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
            
            let response_inner: Result<(), String> = supabase_client
                .delete("test", "1476105020679346924")
                .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }

                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = delete_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }

    /// Tests the `upsert` method of `SupabaseClient`.
    #[tokio::test]
    async fn upsert_string() {
        
        /// Performs a select_filter operation in an isolated scope.
        async fn upsert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
    
        let id: String = "8826759220049045588".to_string();
        let email: String = "floris@xylex.ai".to_string();

        // Usage example
        let response_inner= supabase_client
            .upsert(
                "test",
                &id,
                json!({
                    "email": email
                }),
            )
            .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }

                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = upsert_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }


    /// Tests the `upsert` method of `SupabaseClient`.
    #[tokio::test]
    async fn upsert_numeric() {
        
        /// Performs a select_filter operation in an isolated scope.
        async fn upsert_inner(supabase_client: SupabaseClient) -> Result<(), String> {
            // Usage example
    
        let id: String = "8826759220049045588".to_string();
        let email: i64 = 1234;

        // Usage example
        let response_inner= supabase_client
            .upsert(
                "test",
                &id,
                json!({
                    "email": email
                }),
            )
            .await;

            match response_inner {
                Ok(response_inner) => {
                    println!("Response: {:?}", response_inner);
                    Ok(())
                }

                Err(error) => {
                    println!("Error: {:?}", error);
                    Err(error)
                }
            }
        }

        let supabase_client: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m", e);
                return;
            }
        };
        let response: Result<(), String> = upsert_inner(supabase_client).await;

        assert_eq!(response.is_ok(), true);
    }
}
