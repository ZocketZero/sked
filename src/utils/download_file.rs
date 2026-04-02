//! This module provides a utility function for downloading files from a given URL.

use anyhow::{Ok, anyhow};
use tokio::fs;

/// Downloads a file from a URL and saves it to a specified output path.
///
/// The function extracts the filename from the URL and appends it to `out_path`.
///
/// # Arguments
///
/// * `url` - The URL of the file to download.
/// * `out_path` - The directory path where the file will be saved.
///
/// # Example
///
/// ```
/// # use tokio_test;
/// # use crate::sked::utils::download_file;
/// # tokio_test::block_on(async {
/// let url = "https://crates.io/assets/cargo.png".to_string();
/// let out_path = "/tmp/".to_string();
/// download_file(url, out_path).await.unwrap();
/// # })
/// ```
pub async fn download_file(url: String, out_path: String) -> anyhow::Result<()> {
    // Create a new reqwest client.
    let client = reqwest::Client::new();
    // Send a GET request to the URL.
    let res = client.get(&url).send().await?;
    // Extract the filename from the URL.
    let file_name = match url.split('/').last() {
        Some(fname) => fname,
        None => return Err(anyhow!("Failed to create file name from URL")),
    };
    // Asynchronously write the response bytes to the specified file path.
    fs::write(out_path + file_name, res.bytes().await?).await?;

    Ok(())
}
