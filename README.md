# supabase_rs

`supabase_rs` is an extremely light weight Supabase SDK for interacting with it's database.

I'm actively covering the entire Supabase API including Auth, Realtime, Storage etc

## Feature flags
- **`nightly`**: Enables the `GraphQL` module to interact without REST.
- **`storage`**: Enables the `Storage` module to interact with Supabase Storage.
- **`rustls`**: Forces the client into using `rustls` over `OpenSSL`.

## Nightly build
If you want to use GraphQL early you can enable the `nightly` flag, this is NOT production ready obviously.

Nightly WILL send a warning message, to disable the `nightly` warning message

disable it in your `.env` as such:
```env
SUPABASE_RS_NO_NIGHTLY_MSG=true
```


* Note 
GraphQL and REST can be used together as the Client is shared from wherever it was authenticated initially.


## Database Features

- [x] Updating
- [x] Inserting
- [x] Inserting if unique
- [ ] Bulk Inserting
- [ ] Upserting
- [ ] Bulk Upserting
- [x] Delete (only per ID)
- [x] Select
- [x] Applying Filters
- [x] Counting total records

## Advanced Filtering over `select()`

- [x] Column is equal to a value
- [x] Column is not equal to a value
- [x] Column is greater than a value
- [x] Column is less than a value
- [x] Column is greater than or equal to a value
- [x] Column is less than or equal to a value
- [ ] Order the results
- [ ] Limit the number of rows returned
- [ ] Retrieve as a CSV

## Storage

- [x] Downloading a file from a public bucket
- [x] Saving a file
- [ ] Saving a file to a private bucket
- [ ] Uploading a file
- [ ] Generating a signed url
- [ ] Deleting a file

## GraphQL
- [x] Query validation
- [x] Calling a Query
- [ ] Calling a mutating Query
- [ ] Response parsing


## Auth

// coming soon //

## Realtime

// coming soon //


# Supabase SDK for Rust

This is an unofficial Rust SDK for [Supabase](https://supabase.io/), since there is no official SDK for Rust yet.

## Features
- [**`Insert`**](#insert): Add new rows to a table.
- [**`Insert if unique`**](#insert-if-unique): Add a new row only if it does not violate a UNIQUE constraint.
- [**`Update`**](#update): Modify existing rows in a table based on a unique identifier.
- [**`Select`**](#select): Insert a new row into a table if it does not exist, or update it if it does.
- [**`Select with count`**](#select-with-count): Select rows from a table and count the number of rows that match the filter criteria.
- [**`Select with filter`**](#select-with-filter): Select rows from a table based on a filter criteria.
- [**`Select with filter and count`**](#selecting-with-filter-and-count): Select rows from a table based on a filter criteria and count the number of rows that match the filter criteria.
- [**`Delete`**](#delete): Delete a row from a table based on a unique identifier.

## Graphql features
- [**`Query request`**](#query-request): Runs a GraphQL query to supabase


## Feature flags
- **`storage`**: Enables the `Storage` module to interact with Supabase Storage.

## Cargo.toml
```toml
[dependencies]
supabase-rs = "0.3.1"

// With the [storage] feature
supabase-rs = { version = "0.3.1", features = ["storage"] }
```
//!
## Usage
First make sure you have initialized the Supabase Client
[Initalizing the SupabaseClient](#initialize-the-supabase-client)
//!
## Authentication
The Supabase Client is initialized with the Supabase URL and the Supabase Key.
Which are environment variables that can be set in a `.env` file under the following names or any other
```
SUPABASE_URL=
SUPABASE_KEY=
```

## Examples

### Initialize the Supabase Client
 ```rust
use supabase_rs::SupabaseClient;

use dotenv::dotenv;
use std::env::var;

async fn initialize_supabase_client() -> SupabaseClient {
    dotenv().ok(); // Load the .env file

    let supabase_client: SupabaseClient = SupabaseClient::new(
        var("SUPABASE_URL").unwrap(),
        var("SUPABASE_KEY").unwrap()
        );

    supabase_client
}
```
This will initialize the Supabase Client with the Supabase URL and the Supabase Key, and return the Supabase Client to be passed to other methods.

More examples will be present in the docs