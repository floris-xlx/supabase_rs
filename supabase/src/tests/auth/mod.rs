#[cfg(feature = "auth")]
#[cfg(test)]
mod auth {
    use crate::tests::methods::init::init;
    use crate::SupabaseClient;

    #[tokio::test]
    async fn auth() -> anyhow::Result<()> {
        let supabase: SupabaseClient = match init().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!(
                    "\x1b[31mFailed to initialize Supabase client: {:?}\x1b[0m",
                    e
                );
                return Ok(());
            }
        };

        let _auth_client = supabase.auth();
        // dbg!(_auth_client);
        Ok(())
    }
}
