pub mod base;

#[cfg(test)]
pub fn create_test_supabase_client() -> Result<crate::SupabaseClient, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let supabase_url = std::env::var("SUPABASE_URL")?;
    let supabase_key = std::env::var("SUPABASE_KEY")?;

    Ok(crate::SupabaseClient::new(supabase_url, supabase_key))
}
