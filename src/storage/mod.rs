//! This module provides a struct and methods for interacting with Supabase Storage.
//!
//! # Features
//!
//! - Downloading files from Supabase Storage
//! - Saving files to the local system
//!
//! # Table of Contents
//!
//! - [SupabaseStorage](#supabasestorage)
//! - [download](./download/index.html)
//! - [save](./download/index.html)
//!
//! # Usage
//! First make sure you have initialized the Supabase Client
//! [Initalizing the SupabaseClient](../struct.SupabaseClient.html)
//!
//! ### Downloading a file to a `bytes` object
//! ```rust
//! use supabase_rs::SupabaseStorage;
//!
//! let storage = SupabaseStorage {
//!     supabase_url: "https://example.com".to_string(),
//!     bucket_name: "bucket".to_string(),
//!     filename: "file.txt".to_string(),
//! };
//!
//! let bytes = storage.download().await.unwrap();
//! ```
//! 
//! ### Downloading a file directly to the local system
//! ```rust
//! use supabase_rs::SupabaseStorage;
//! 
//! let storage = SupabaseStorage {
//!    supabase_url: "https://example.com".to_string(),
//!    bucket_name: "bucket".to_string(),
//!    filename: "file.txt".to_string(),
//! };
//! 
//! storage.save("local_file.txt").await.unwrap();  
//! ```
//! 
#![cfg(feature = "storage")]

pub mod download;

/// A struct for interacting with Supabase Storage.
#[derive(Debug, Clone)]
pub struct SupabaseStorage {
    pub supabase_url: String,
    pub bucket_name: String,
    pub filename: String,
}