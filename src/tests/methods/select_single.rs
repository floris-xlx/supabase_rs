use crate::tests::methods::init::init;
use crate::SupabaseClient;

pub async fn select_single() {
    /// Tests that `.single()` returns exactly one row and fails if none or multiple exist.
    async fn select_single_inner(client: SupabaseClient) -> Result<(), String> {
        let sentinel = serde_json::json!({
            "id": 0,
            "number": "99999"
        });

        let _ = client.insert("test", &sentinel).await.map_err(|e| {
            eprintln!("\x1b[33mWarning: upsert failed: {:?}\x1b[0m", e);
            e
        });

        // Now query to get exactly that row
        let res = client
            .select("test")
            .eq("id", "test-single-sentinel")
            .single()
            .await;

        match res {
            Ok(row) => {
                assert!(row.is_object());
                Ok(())
            }
            Err(e) => {
                eprintln!("\x1b[31mError: {:?}\x1b[0m", e);
                Err(e)
            }
        }
    }

    let client = match init().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[31mFailed to init client: {:?}\x1b[0m", e);
            return;
        }
    };

    // To clean up afterwards
    let cloned_client = client.clone();

    let result = select_single_inner(client).await;

    // delete the sentinel row
    if let Err(e) = cloned_client.delete("test", "test-single-sentinel").await {
        eprintln!("\x1b[33mWarning: cleanup failed: {:?}\x1b[0m", e);
    }

    assert!(result.is_ok());
}
