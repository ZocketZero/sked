//! This module provides a utility for writing data to files.

use tokio::io::AsyncWriteExt;

/// A struct to encapsulate file writing operations.
pub struct WriteFile {
    /// The path to the file.
    path: String,
}

impl WriteFile {
    /// Creates a new `WriteFile` instance with the specified file path.
    pub fn new(path: String) -> Self {
        WriteFile { path }
    }

    /// Synchronously writes an entire string slice to the file, overwriting existing content.
    pub fn write(&self, content: &str) -> std::io::Result<()> {
        std::fs::write(&self.path, content)
    }

    /// Asynchronously appends a byte slice to the end of the file.
    /// If the file does not exist, it will be created.
    pub async fn append(&self, data: &[u8]) -> anyhow::Result<()> {
        // Open the file in append mode.
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&self.path)
            .await?;
        // Write all data to the file.
        file.write_all(data).await?;

        Ok(())
    }
}
