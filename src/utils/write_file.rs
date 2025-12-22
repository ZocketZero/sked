use tokio::io::AsyncWriteExt;

pub struct WriteFile {
    path: String,
}

impl WriteFile {
    pub fn new(path: String) -> Self {
        WriteFile { path }
    }
    pub fn write(&self, content: &str) -> std::io::Result<()> {
        std::fs::write(&self.path, content)
    }
    pub async fn append(&self, data: &[u8]) -> anyhow::Result<()> {
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.path.clone())
            .await?;
        file.write_all(data).await?;

        Ok(())
    }
}
