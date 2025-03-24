use crate::Result;
use crate::SupabaseClient;
use dotenv::dotenv;
use std::env::var;

pub async fn init() -> Result<SupabaseClient> {
    dotenv().ok();

    let supabase_url: String = var("SUPABASE_URL")?;
    let supabase_key: String = var("SUPABASE_KEY")?;

    SupabaseClient::new(supabase_url, supabase_key)
}
