use supabase_rs_auth::{AuthClient, IdType};

/// An integration test that signs up a user, signs them in, and then signs them out.
#[tokio::test]
async fn sign_up_in_out() -> anyhow::Result<()> {
    fn get_auth_client() -> anyhow::Result<AuthClient> {
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::try_init().ok();

        let supabase_url = std::env::var("SUPABASE_URL")?;
        let supabase_key = std::env::var("SUPABASE_KEY")?;

        AuthClient::new(&supabase_url, &supabase_key)
    }

    let auth_client = match get_auth_client() {
        Ok(client) => client,
        Err(e) => {
            println!("Cannot create an auth client. Most probably SUPABASE_URL and/or SUPABASE_KEY env vars are not exported: {e}");
            return Ok(());
        }
    };

    let email = "it_test@supabase.rs".to_string();
    let session = auth_client
        .signup(IdType::Email(email.clone()), "supabase", None)
        .await?;

    // Check if the user was created
    assert_eq!(session.user.unwrap().email, Some(email.clone()));

    // but the session is not set yet
    assert_eq!(auth_client.session(), None);

    let session = auth_client
        .signin_with_password(IdType::Email(email.clone()), "supabase")
        .await?;

    // Check if the user was signed in
    assert_eq!(session.user.clone().unwrap().email, Some(email.clone()));

    // and that the client is authenticated
    assert_eq!(auth_client.session(), Some(session));

    auth_client.signout().await?;

    // check that the client is not authenticated anymore
    assert_eq!(auth_client.session(), None);

    Ok(())
}
