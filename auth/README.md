# supabase_auth_rs

A Rust client library for Supabase Authentication. This crate provides a type-safe interface to interact with Supabase Auth services, making it easy to integrate Supabase authentication into your Rust applications.

## Features

- 🔐 Complete authentication flow support
- 👤 User management (signup, signin, logout)
- 🔄 Token management (refresh tokens)
- 📱 Multiple authentication methods (email/password, phone)
- 🔍 User data retrieval
- ⚡ Async/await support
- 🦀 Type-safe API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
supabase_auth_rs = "0.1.0"
```

## Usage

### Creating a client

```rust
use supabase_auth_rs::AuthClient;

let client = AuthClient::new(
    "YOUR_SUPABASE_URL",
    "YOUR_SUPABASE_ANON_KEY"
)?;
```

### User Authentication

#### Sign Up
```rust
use supabase_auth_rs::{AuthClient, IdType};

// Sign up with email
let (user, access_token) = client.signup(
    IdType::Email("user@example.com".to_string()),
    "password123".to_string(),
    None
).await?;

// Sign up with phone
let (user, access_token) = client.signup(
    IdType::PhoneNumber("+1234567890".to_string()),
    "password123".to_string(),
    None
).await?;
```

#### Sign In
```rust
// Sign in with email
let token_response = client.signin_with_password(
    IdType::Email("user@example.com".to_string()),
    "password123".to_string()
).await?;

// Sign in with phone
let token_response = client.signin_with_password(
    IdType::PhoneNumber("+1234567890".to_string()),
    "password123".to_string()
).await?;
```

### Token Management

```rust
// Refresh an access token
let new_token_response = client.refresh_token(&refresh_token).await?;

// Logout
client.logout(&access_token).await?;
```

### User Management

```rust
// Get user details
let user = client.get_user(&user_id).await?;

// Delete user
client.hard_delete_user(user_id).await?;
```

## Error Handling

The library uses a custom `AuthError` type that covers various authentication scenarios:

```rust
pub enum AuthError {
    NotAuthorized,
    InvalidParameters,
    Http,
    Internal,
    NotFound,
    GeneralError,
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
