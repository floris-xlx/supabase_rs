use crate::SupabaseClient;
use std::env::var;
use dotenv::dotenv;

pub async fn init() -> Result<SupabaseClient, Box<dyn std::error::Error>> {
    dotenv().ok();

    let supabase_url: String = var("SUPABASE_URL")?;
    let supabase_key: String = var("SUPABASE_KEY")?;

    Ok(SupabaseClient::new(supabase_url, supabase_key))
}