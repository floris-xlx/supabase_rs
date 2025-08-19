//! # Supabase Storage Operations
//!
//! This module provides comprehensive file management capabilities for Supabase Storage buckets.
//! It supports downloading files to memory or disk with built-in error handling and progress tracking.
//!
//! > **📋 Requirement**: Enable the `storage` feature in your `Cargo.toml` to use this module.
//!
//! ## 🎯 Core Features
//!
//! - **[`download`](SupabaseStorage::download)**: Download files to memory as bytes
//! - **[`save`](SupabaseStorage::save)**: Download files directly to disk
//! - **Error Handling**: Comprehensive error reporting for failed operations
//! - **Flexible Paths**: Support for nested folder structures in buckets
//!
//! ## 🏗️ Storage Architecture
//!
//! Supabase Storage is organized hierarchically:
//! ```text
//! Project
//! └── Buckets (e.g., "avatars", "documents")
//!     └── Files (e.g., "user-123/profile.jpg")
//! ```
//!
//! ## 📖 Usage Examples
//!
//! ### Basic File Download
//!
//! ```rust,no_run
//! use supabase_rs::storage::SupabaseStorage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize storage client
//! let storage = SupabaseStorage {
//!     supabase_url: std::env::var("SUPABASE_URL")?,
//!     bucket_name: "avatars".to_string(),
//!     filename: "user-123/profile.jpg".to_string(),
//! };
//!
//! // Download to memory
//! let file_bytes = storage.download().await?;
//! println!("Downloaded {} bytes", file_bytes.len());
//!
//! // Save directly to disk
//! storage.save("./downloads/profile.jpg").await?;
//! println!("File saved to disk");
//! # Ok(())
//! # }
//! ```
//!
//! ### Batch File Operations
//!
//! ```rust,no_run
//! use supabase_rs::storage::SupabaseStorage;
//! use futures::future::try_join_all;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let base_url = std::env::var("SUPABASE_URL")?;
//! let files = vec!["doc1.pdf", "doc2.pdf", "doc3.pdf"];
//!
//! // Download multiple files concurrently
//! let downloads: Vec<_> = files.into_iter().map(|filename| {
//!     let storage = SupabaseStorage {
//!         supabase_url: base_url.clone(),
//!         bucket_name: "documents".to_string(),
//!         filename: filename.to_string(),
//!     };
//!     storage.download()
//! }).collect();
//!
//! let results = try_join_all(downloads).await?;
//! println!("Downloaded {} files", results.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Error Handling
//!
//! ```rust,no_run
//! # use supabase_rs::storage::SupabaseStorage;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let storage = SupabaseStorage {
//!     supabase_url: std::env::var("SUPABASE_URL")?,
//!     bucket_name: "private-docs".to_string(),
//!     filename: "confidential.pdf".to_string(),
//! };
//!
//! match storage.download().await {
//!     Ok(bytes) => {
//!         println!("✅ Downloaded {} bytes", bytes.len());
//!     },
//!     Err(err) => {
//!         if err.to_string().contains("404") {
//!             println!("📄 File not found");
//!         } else if err.to_string().contains("403") {
//!             println!("🔒 Access denied - check bucket permissions");
//!         } else {
//!             println!("❌ Download failed: {}", err);
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## 🛡️ Security Considerations
//!
//! - **Bucket Policies**: Ensure proper RLS policies are configured
//! - **File Paths**: Validate file paths to prevent directory traversal
//! - **Size Limits**: Consider implementing download size limits
//! - **Authentication**: Use appropriate API keys for bucket access levels
//!
//! ## ⚡ Performance Tips
//!
//! - **Concurrent Downloads**: Use `try_join_all` for multiple file operations
//! - **Streaming**: For large files, consider streaming directly to disk
//! - **Caching**: Cache frequently accessed files locally
//! - **Compression**: Use compressed formats when possible
#![cfg(feature = "storage")]

pub mod download;

/// Client for interacting with Supabase Storage buckets.
///
/// Provides a simple interface for downloading files from Supabase Storage.
/// Each instance is configured for a specific file in a specific bucket.
///
/// # Configuration
///
/// - **`supabase_url`**: Your project's Supabase URL
/// - **`bucket_name`**: The storage bucket containing the file
/// - **`filename`**: The file path within the bucket (supports nested paths)
///
/// # File Path Structure
///
/// File paths can include folders and subfolders:
/// - Simple: `"avatar.jpg"`
/// - Nested: `"users/123/documents/resume.pdf"`
/// - Timestamped: `"uploads/2024/01/15/image.png"`
///
/// # Examples
///
/// ```rust,no_run
/// use supabase_rs::storage::SupabaseStorage;
///
/// // Simple file reference
/// let avatar = SupabaseStorage {
///     supabase_url: "https://project.supabase.co".to_string(),
///     bucket_name: "avatars".to_string(),
///     filename: "user-123.jpg".to_string(),
/// };
///
/// // Nested file reference
/// let document = SupabaseStorage {
///     supabase_url: "https://project.supabase.co".to_string(),
///     bucket_name: "documents".to_string(),
///     filename: "users/123/contracts/agreement.pdf".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct SupabaseStorage {
    /// The base URL of your Supabase project
    pub supabase_url: String,
    /// The name of the storage bucket
    pub bucket_name: String,
    /// The file path within the bucket (supports nested paths)
    pub filename: String,
}
