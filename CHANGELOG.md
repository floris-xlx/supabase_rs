# Changelog

## 0.1.0 (2024-07-10)


### Features

* add `bulk_insert` function ([33f30e3](https://github.com/floris-xlx/supabase_rs/commit/33f30e36d3e62e2884912b615b0140c2bf3542df))
* Add error message for table name not ending with Collection ([27dd186](https://github.com/floris-xlx/supabase_rs/commit/27dd1869d879663cb97f96f309c24f1d4440e1fe))
* add insert_without_defined_key method ([dc8e0ed](https://github.com/floris-xlx/supabase_rs/commit/dc8e0ed002ddc535e3d1f9bc0d851fb67336d38a))
* Add support for selecting specific columns in the select query ([309d8dc](https://github.com/floris-xlx/supabase_rs/commit/309d8dcb9c63259f48b801cb9400a84c7c6bfffe))
* allow users to select column names to reference when updating ([b2de629](https://github.com/floris-xlx/supabase_rs/commit/b2de62944a8f682b6a9d2664bcf2926617420919))
* implement missing Query methods ([4d0b9ba](https://github.com/floris-xlx/supabase_rs/commit/4d0b9ba13571063a0412d86030ea650fbaea2bf1))
* Query::build() takes filters and sort ([766592d](https://github.com/floris-xlx/supabase_rs/commit/766592dfd8097441dfdd3711f90d1bf62dfcc888))


### Bug Fixes

* abstracted all tests into seperate methods with files ([d2c60b8](https://github.com/floris-xlx/supabase_rs/commit/d2c60b8e24a37c11f3199f9358b813fe357b20f6))
* derive Default instead of manually implementing it ([77299ad](https://github.com/floris-xlx/supabase_rs/commit/77299ad9816917013c799b1aaac0c958a8149818))
* ignore or modify some doctests ([e6a2e03](https://github.com/floris-xlx/supabase_rs/commit/e6a2e0361e7e5001b57e57cf6158c3080a0492ab))
* implement std::fmt::Display for Filter instead of creating to_string() ([0bce354](https://github.com/floris-xlx/supabase_rs/commit/0bce35484e303c51404fe60bf597aeb7a46a295a))
* use `assert!()` instead of `assert_eq!()` for bool literal comparison ([efd10df](https://github.com/floris-xlx/supabase_rs/commit/efd10df39eb58b88d38f3966dcf8e2112a5ab1a8))
