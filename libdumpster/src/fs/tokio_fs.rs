use super::*;
use std::{io, path::PathBuf};
use tokio::fs;

pub struct TokioFileSystem {
    mount_point: PathBuf,
}

#[async_trait]
impl FileSystem for TokioFileSystem {
    async fn mount(mount_point: &str) -> Result<Self, FileSystemError>
    where
        Self: Sized,
    {
        let mount_point = PathBuf::from(mount_point);
        Ok(Self { mount_point })
    }

    async fn register_class(&self, name: &str) -> Result<(), FileSystemError> {
        let mut dir = self.mount_point.clone();
        dir.push(name);
        fs::create_dir(&dir)
            .await
            .map_err(|e| FileSystemError::Internal(e.to_string()))
    }

    async fn new_object(&self, class_name: &str, object_name: &str) -> Result<(), FileSystemError> {
        let mut dir = self.mount_point.clone();
        dir.push(class_name);
        dir.push(object_name);
        fs::create_dir(&dir)
            .await
            .map_err(|e| FileSystemError::Internal(e.to_string()))
    }

    async fn store(
        &self,
        class_name: &str,
        object_name: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<(), FileSystemError> {
        let mut dir = self.mount_point.clone();
        dir.push(class_name);
        dir.push(object_name);
        dir.push(name);
        fs::write(&dir, bytes).await.map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                FileSystemError::NotFound
            } else {
                FileSystemError::Internal(e.to_string())
            }
        })
    }
}
