use crate::SupabaseClient;

pub fn endpoint(client: &SupabaseClient) -> String {
    format!("{}/graphql/v1", client.url)
}

