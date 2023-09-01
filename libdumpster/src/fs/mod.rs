//! Generic file system for dependency injection.
//!
//! To keep things flexible, "objects" can be created within a "class".
//! These objects then contain "files".
//!
//! On a system file system, this translates to:
//!
//! ```
//! class_1/
//!     object_1/
//!         file.png
//!         file.jpeg
//!     object_2/
//!         file.png
//!         file.jpeg
//! ```
//!
use async_trait::async_trait;

pub mod mock_fs;
pub mod tokio_fs;

#[derive(Debug, Clone)]
pub enum FileSystemError {
    NotFound,
    Internal(String),
}

#[async_trait]
pub trait FileSystem {
    async fn mount(mount_point: &str) -> Result<Self, FileSystemError>
    where
        Self: Sized;
    async fn register_class(&self, name: &str) -> Result<(), FileSystemError>;
    async fn new_object(&self, class_name: &str, object_name: &str) -> Result<(), FileSystemError>;
    async fn store(
        &self,
        class_name: &str,
        object_name: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<(), FileSystemError>;
}
