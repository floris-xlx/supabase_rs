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
- [x] Bulk Inserting
- [x] Upserting
- [ ] Bulk Upserting
- [x] Delete (only per ID)
- [x] Select
- [x] Select specific columns
- [x] Applying Filters
- [x] Counting total records
- [x] Type generator from schema

## Advanced Filtering over `select()`

- [x] Column is equal to a value
- [x] Column is not equal to a value
- [x] Column is greater than a value
- [x] Column is less than a value
- [x] Column is greater than or equal to a value
- [x] Column is less than or equal to a value
- [x] Order the results
- [x] Limit the number of rows returned
- [x] Use text search
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

<!-- cargo-rdme start -->

### Supabase SDK for Rust

This is an unofficial Rust SDK for [Supabase](https://supabase.io/), since there is no official SDK for Rust yet.

#### Features
- [**`Insert`**](#insert): Add new rows to a table.
- [**`Insert if unique`**](#insert-if-unique): Add a new row only if it does not violate a UNIQUE constraint.
- [**`Update`**](#update): Modify existing rows in a table based on a unique identifier.
- [**`Select`**](#select): Insert a new row into a table if it does not exist, or update it if it does.
- [**`Select with count`**](#select-with-count): Select rows from a table and count the number of rows that match the filter criteria.
- [**`Select with filter`**](#select-with-filter): Select rows from a table based on a filter criteria.
- [**`Select with filter and count`**](#selecting-with-filter-and-count): Select rows from a table based on a filter criteria and count the number of rows that match the filter criteria.
- [**`Delete`**](#delete): Delete a row from a table based on a unique identifier.

#### Graphql features
- [**`Query request`**](#query-request): Runs a GraphQL query to supabase

#### Feature flags
- **`storage`**: Enables the `Storage` module to interact with Supabase Storage.
- **`nightly`**: Enables the nightly features.
- **`rustls`**: Forces the client into using `rustls` over `OpenSSL`.

#### Nightly Build
- **`nightly`**: Enables the `GraphQL` module to interact with Supabase GraphQL API.

Nightly features are not stable and may break at any time without notice, so use with caution.

Nightly WILL send a warning message, to disable the `nightly` warning message

disable it in your `.env` as such:
```env
SUPABASE_RS_NO_NIGHTLY_MSG=true
```

#### Cargo.toml
```toml
[dependencies]
supabase-rs = "..."

// With the [storage] feature
supabase-rs = { version = "...", features = ["storage"] }
```

#### Usage
First make sure you have initialized the Supabase Client
[Initalizing the SupabaseClient](#initialize-the-supabase-client)

#### Authentication
The Supabase Client is initialized with the Supabase URL and the Supabase Key.
Which are environment variables that can be set in a `.env` file under the following names or any other
```rust
SUPABASE_URL=
SUPABASE_KEY=
```

#### Examples

##### Initialize the Supabase Client
 ```rust
use supabase_rs::SupabaseClient;

use dotenv::dotenv;
use std::env::var;

async fn initialize_supabase_client() -> SupabaseClient {
    dotenv().ok(); // Load the .env file

    let supabase_client: SupabaseClient = SupabaseClient::new(
        var("SUPABASE_URL").unwrap(),
        var("SUPABASE_KEY").unwrap()
        ).unwrap();

        supabase_client
   }
```
This will initialize the Supabase Client with the Supabase URL and the Supabase Key, and return the Supabase Client to be passed to other methods.

##### Insert
This will insert a new row into the `test` table with the value `value_test` in the `dog` column.

```rust
// i know the imports are self explanatory but it makes it easier for beginners:)
use serde_json::json;
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
    "your_supabase_url", "your_supabase_key"
);

async fn insert_example(
   client: SupabaseClient
) -> Result<(), String> {
    let insert_result = client
        .insert(
            "test",
            json!({
                "dog": "value_test"
            }),
       ).await;
```

##### Insert if unique
This will insert a new row into the `test` table with the value `value_test` in the `dog` column if the value is unique.
It's a drop-in replacement for `insert` without relying on Supabase's unique constraints on the database.

```rust
use serde_json::json;
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
    "your_supabase_url", "your_supabase_key"
);

async fn insert_example(
   client: SupabaseClient
) -> Result<(), String> {
    let insert_result = client
        .insert_if_unique(
            "test",
            json!({
                "dog": "value_test"
            }),
       ).await;
```

##### Update
This will update the row in the `test` table with the value `value_test` in the `dog` column where the `id` is `1`.

```rust
use serde_json::json;
use supabase_rs::SupabaseClient;

let client = SupabaseClient::new(
   "your_supabase_url", "your_supabase_key"
);

async fn update_example(
  client: SupabaseClient
) -> Result<(), String> {
   let update_result = client
      .update_with_column_name(
         "table_name", // the table name
         "column_name",    // the column name to filter by
         "id", // the value to filter by (can be any value to use as key)
         json!({
           "dog": "value_test"  // the new value
         }),
     ).await;
```

##### Select
This will return all `dog` rows where the value is `scooby` in the `animals` table

```rust
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
   "your_supabase_url", "your_supabase_key"
);

async fn select_scooby(
   supabase_client: SupabaseClient
) -> Result<(), String> {

let data: Result<Vec<Value>, String> = supabase_client
   .select("animals")
   .eq("dog", "scooby")
   .execute()
   .await;
```

##### Select on specific column
This will return all the `dog` rows where the value is `scooby` in the `animals` table and only return the `dog` column.

```rust
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
   "your_supabase_url", "your_supabase_key"
);

async fn select_scooby(
   supabase_client: SupabaseClient
) -> Result<(), String> {

let data: Result<Vec<Value>, String> = supabase_client
   .select("animals")
   .columns(["dog"].to_vec())
   .eq("dog", "scooby")
   .execute()
   .await;
```

##### Select with Count
<div class="warning">Counting is very expensive and will be alot slower, so only use it if you need it </div>

This will return all `dog` rows where the value is `scooby` in the `animals` table and count the number of rows that match the filter criteria.

```rust
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
  "your_supabase_url", "your_supabase_key"
);

async fn select_scooby_with_count(
  supabase_client: SupabaseClient
) -> Result<(), String> {
 let data: Result<Vec<Value>, String> = supabase_client
   .select("animals")
   .count()
   .execute()
   .await;
```

##### Select with Filter
This will return all `dog` rows where the value is `scooby` in the `animals` table

```rust
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
  "your_supabase_url", "your_supabase_key"
);

async fn select_scooby_with_filter(
 supabase_client: SupabaseClient
) -> Result<(), String> {
let data: Result<Vec<Value>, String> = supabase_client
    .select("animals")
    .eq("dog", "scooby")
    .execute()
    .await;
```

##### Selecting with Filter and Count
<div class="warning">Counting is very expensive and will be alot slower, so only use it if you need it </div>

This will return all `dog` rows where the value is `scooby` in the `animals` table and count the number of rows that match the filter criteria.
```rust
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
 "your_supabase_url", "your_supabase_key"
);

async fn select_scooby_with_filter_and_count(
supabase_client: SupabaseClient
) -> Result<(), String> {
let data: Result<Vec<Value>, String> = supabase_client
    .select("animals")
    .eq("dog", "scooby")
    .count()
    .execute()
    .await;
```

##### Delete
This will delete the row in the `test` table where the `id` is `1`.

```rust
// i know the imports are self explanatory but it makes it easier for beginners:)
use supabase_rs::SupabaseClient;

// always pass an initialized SupabaseClient to the method
let client = SupabaseClient::new(
  "your_supabase_url", "your_supabase_key"
);

async fn delete_example(
 client: SupabaseClient
) -> Result<(), String> {
let delete_result = client
    .delete("test", "1")
    .await;
```

//! <div class="warning">Experimental features, Not ready for prod!</div>


##### Get ID by Column, Cell values
This will return the ID of the row in the specified table where the column matches the provided email.

```rust
#[tokio::main]
async fn main() {
    // Initialize the Supabase Client
    let supabase_client = SupabaseClient::new("your_supabase_url", "your_supabase_key");

    let email = "example@email.com".to_string();
    let table_name = "users".to_string();
    let column_name = "email".to_string();
    match supabase_client.get_id(email, table_name, column_name).await {
        Ok(id) => println!("Found ID: {}", id),
        Err(e) => println!("Error: {}", e),
    }
}
```


#### Different Operations
- [Insert](./insert/index.html)
- [Update](./update/index.html)
- [Select](./select/index.html)
- [Storage](./storage/index.html)
- [Realtime](./realtime/index.html)
- [Query](./query/index.html)
- [Errors](./errors/index.html)
- [Success](./success/index.html)
- [Tests](./tests/index.html)
- [GraphQL](./graphql/index.html)


#### Update
I'll be adding more methods and enriching the SDK over the next few weeks, for now!

#### Contributers

<!-- cargo-rdme end -->

## Graphql features
- [**`Query request`**](#query-request): Runs a GraphQL query to supabase


## Feature flags
- **`storage`**: Enables the `Storage` module to interact with Supabase Storage.

## Cargo.toml
```toml
[dependencies]
supabase_rs = "0.4.6"

// With the [storage] feature
supabase_rs = { version = "0.4.6", features = ["storage"] }
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

Examples will be present in the docs

## Contributors
- [Hadi](https://github.com/hadi-xlx) - Improved & fixed the schema to type generator
- [Izyuumi](https://github.com/izyuumi) - Improved row ID routing with updating methods
- [koya1616](https://github.com/koya1616) - fixed README
- [strykejern](https://github.com/strykejern) - Refactored for maintainability & fixed a warnings

