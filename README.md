# supabase_rs

An unofficial, lightweight Rust SDK for interacting with the Supabase REST and GraphQL APIs. It focuses on a clean, chainable query-builder, simple CRUD helpers, and optional modules for Storage and Realtime.

- Works with pure REST by default; optional nightly GraphQL support
- Query-Builder for fluent filtering, ordering, limiting, and text search
- Insert, Update, Upsert, Delete helpers
- Optional Storage helpers (feature-flagged)

## Table of contents
- Installation
- Features and flags
- Quickstart
- Database examples
- Storage (optional)
- GraphQL (nightly/experimental)
- Testing
- Contributors

## Installation
Add the crate to your project:

```toml
[dependencies]
supabase-rs = "*"

# With the storage feature
supabase-rs = { version = "*", features = ["storage"] }

# Use rustls instead of OpenSSL
supabase-rs = { version = "*", features = ["rustls"] }
```

Environment variables (recommended via `.env`):

```env
SUPABASE_URL=...
SUPABASE_KEY=...
```

## Features and flags
- `storage`: Enables the `storage` module
- `rustls`: Forces the client to use rustls
- `nightly`: Enables experimental GraphQL support and prints a warning by default. To silence the warning:

```env
SUPABASE_RS_NO_NIGHTLY_MSG=true
```

## Quickstart
```rust
use supabase_rs::SupabaseClient;

fn client() -> SupabaseClient {
    SupabaseClient::new(
        std::env::var("SUPABASE_URL").unwrap(),
        std::env::var("SUPABASE_KEY").unwrap(),
    ).unwrap()
}
```

## Database examples

### Insert
```rust
use serde_json::json;
let client = client();
let id = client.insert("pets", json!({ "name": "scooby" })).await?;
```

### Insert if unique
```rust
use serde_json::json;
let client = client();
let id = client.insert_if_unique("users", json!({ "email": "a@b.com" })).await?;
```

### Update / Upsert
```rust
use serde_json::json;
let client = client();
client.update("pets", "1", json!({ "name": "scooby-doo" })).await?;
client.upsert("pets", "1", json!({ "name": "scooby-doo" })).await?;
```

### Select with filters
```rust
use serde_json::Value;
let client = client();
let rows: Vec<Value> = client
    .select("pets")
    .eq("name", "scooby")
    .limit(10)
    .order("created_at", true)
    .execute()
    .await?;
```

### Select specific columns
```rust
use serde_json::Value;
let client = client();
let rows: Vec<Value> = client
    .from("pets")
    .columns(["id", "name"].to_vec())
    .execute()
    .await?;
```

### Count (expensive)
```rust
let count_rows = client.select("pets").count().execute().await?;
```

## Storage (optional feature)
Enable the `storage` feature to use storage helpers. See docs in `src/storage` for details and examples.

## GraphQL (nightly / experimental)
Enable the `nightly` feature to access GraphQL helpers. This is experimental and not production-ready. REST and GraphQL can be mixed — the client instance is shared.

## Testing
This repository includes both integration-like tests (that talk to a Supabase instance) and pure unit tests. To run tests that hit Supabase, set `SUPABASE_URL` and `SUPABASE_KEY` in your environment.

## Contributors
- [Hadi](https://github.com/hadi-xlx) — Improved & fixed the schema-to-type generator
- [Izyuumi](https://github.com/izyuumi) — Improved row ID routing with updating methods
- [koya1616](https://github.com/koya1616) — README fixes
- [strykejern](https://github.com/strykejern) — Refactoring & warning fixes

