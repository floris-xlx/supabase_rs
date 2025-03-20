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
//! - [download](#download)
//! - [save](#save)

#![cfg(feature = "storage")]

use anyhow::{Error, Result};
use reqwest::{Client, Error as ReqwestError, Response};
use std::fs::File;
use std::io::prelude::*;

use crate::storage::SupabaseStorage;

impl SupabaseStorage {
    /// Downloads a file from Supabase Storage.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use supabase_rs::SupabaseStorage;
    ///
    /// let storage = SupabaseStorage {
    ///     supabase_url: "https://example.com".to_string(),
    ///     bucket_name: "bucket".to_string(),
    ///     filename: "file.txt".to_string(),
    /// };
    ///
    /// let bytes = storage.download().await.unwrap();
    /// ```
    pub async fn download(&self) -> Result<Vec<u8>, ReqwestError> {
        let url: String = format!(
            "{}/storage/v1/object/public/{}/{}",
            self.supabase_url, self.bucket_name, self.filename
        );
        let client: Client = Client::new();
        let response: Response = client.get(&url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Saves a file to the local system.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use supabase_rs::SupabaseStorage;
    ///
    /// let storage = SupabaseStorage {
    ///     supabase_url: "https://example.com".to_string(),
    ///     bucket_name: "bucket".to_string(),
    ///     filename: "file.txt".to_string(),
    /// };
    ///
    /// storage.save("local_file.txt").await.unwrap();
    /// ```
    pub async fn save(&self, file_path: &str) -> Result<(), Error> {
        let bytes: Vec<u8> = self.download().await.map_err(Error::new)?;

        let mut file: File = File::create(file_path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}
