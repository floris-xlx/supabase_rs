This directory contains integration-like tests that require a live Supabase project.

To run them locally, export the following environment variables or create a `.env` file:

```
SUPABASE_URL=...
SUPABASE_KEY=...
```

If you only want to run pure unit tests (no network), you can filter by `unit_`:

```
cargo test unit_
```
# adding tests

base.rs will contain a call to each test in the /methods dir

to add a new test:
- add a new `METHOD_NAME.rs` in /methods
- add the `.rs` file to `mod.rs` 
- import the test in `base.rs` 
- run the test in the `methods` mod under a `#[tokio::test]` macro
- ?? success
