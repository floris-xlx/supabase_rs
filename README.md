# supabase_rs

`supabase_rs` is an extremely light weight Supabase SDK for interacting with it's database.

I'm actively covering the entire Supabase API including Auth, Realtime, Storage etc

## Feature flags
- **`storage`**: Enables the `Storage` module to interact with Supabase Storage.


## Database Features

- [x] Updating
- [x] Inserting
- [x] Inserting if unique
- [ ] Bulk Inserting
- [ ] Upserting
- [ ] Bulk Upserting
- [ ] Delete
- [x] Select
- [*] Applying Filters

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


## Auth

// coming soon //

## Realtime

// coming soon //



### Quickstart
Add the following dependency to your toml file:
```
[dependencies]
supabase_rs = "0.2.3"

// With the [storage] feature flag
supabase-rs = { version = "0.2.3", features = ["storage"] }
```

### Docs
[Docs](https://docs.rs/supabase_rs/latest/supabase_rs/)

## Support
If you need any specific features added or have any requests either contact me on
discord or open an issue on the [Repo](https://github.com/floris-xlx/supabase_rs)