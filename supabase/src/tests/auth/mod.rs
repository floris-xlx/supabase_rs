#[cfg(feature = "auth")]
#[cfg(test)]
mod auth {
    use crate::tests::create_test_supabase_client;

    #[tokio::test]
    async fn auth() -> anyhow::Result<()> {
        let supabase = create_test_supabase_client()?;
        let _auth_client = supabase.auth();
        // dbg!(_auth_client);
        Ok(())
    }
}
