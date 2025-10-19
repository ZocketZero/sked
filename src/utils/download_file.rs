use anyhow::{Ok, anyhow};
use tokio::fs;

/// Download file from url.
///
/// ```
/// use tokio_test;
/// use crate::htils::utils::download_file;
/// # tokio_test::block_on(async {
/// let url = "https://crates.io/assets/cargo.png".to_string();
/// let out_path = "/tmp/".to_string();
/// download_file(url, out_path).await.unwrap();
/// # })
///
/// ```
pub async fn download_file(url: String, out_path: String) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?;
    let file_name = match url.split("/").last() {
        Some(fname) => fname,
        None => return Err(anyhow!("Failed created file name")),
    };
    fs::write(out_path + file_name, res.bytes().await?).await?;

    Ok(())
}
