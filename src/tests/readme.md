# adding tests

base.rs will contain a call to each test in the /methods dir

to add a new test:
- add a new `METHOD_NAME.rs` in /methods
- add the `.rs` file to `mod.rs` 
- import the test in `base.rs` 
- run the test in the `methods` mod under a `#[tokio::test]` macro
- ?? success
