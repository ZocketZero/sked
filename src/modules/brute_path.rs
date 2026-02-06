//! This module provides the functionality for the `brute-path` command,
//! which brute-forces website directories using a wordlist.

use crate::utils::{download_file, Log, RunCommand, WordlistType, WriteFile};
use clap::Args;

/// Defines the command-line arguments for the `brute-path` subcommand.
#[derive(Args)]
pub struct BrutePathArg {
    /// Target URL with `:path:` as a placeholder for wordlist entries.
    #[arg(short, long)]
    pub url: String,
    /// Wordlist source, which can be a file path or a number range (e.g., "1-100").
    #[arg(short, long)]
    pub wordlist: String,
    /// Specifies which HTTP status codes to accept as valid.
    /// Can be a comma-separated list (e.g., "200,301"), "all", or "ok" (for 200-299).
    #[arg(short, long, default_value = "ok")]
    pub accept_status: Option<String>,
    /// If set, found files or pages will be downloaded.
    #[arg(short, long, default_value_t = false)]
    pub download: bool,
    /// If set, requests will be sent in parallel to speed up the process.
    #[arg(short, long, default_value_t = false)]
    pub parallel: bool,
    /// The output directory to save results or downloaded files.
    #[arg(short, long, default_value = "./")]
    pub out: Option<String>,
}

impl RunCommand for BrutePathArg {
    /// Executes the brute-path logic based on the provided arguments.
    async fn run(&self) {
        let accept_status = self.accept_status.clone().unwrap_or_default();
        // Create and run a new BrutePath instance.
        BrutePath::new(
            self.url.clone(),
            &self.wordlist,
            &accept_status,
            self.download,
            self.parallel,
            self.out.clone(),
        )
        .run()
        .await;
    }
}

/// Represents the types of accepted HTTP status codes.
#[derive(Default, Clone)]
pub enum AcceptStatus {
    /// Accept all HTTP status codes.
    All,
    /// Accept a specific list of status codes.
    Specific(Vec<u16>),
    #[default]
    /// Accept status codes in the 200-299 range (i.e., success).
    Ok,
    /// Do not accept any status codes (effectively disabling status-based filtering).
    None,
}

impl AcceptStatus {
    /// Parses a string input into an `AcceptStatus` enum variant.
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        if input.is_empty() {
            Ok(AcceptStatus::None)
        } else if input.to_lowercase() == "all" || input == "-" {
            Ok(AcceptStatus::All)
        } else if input.to_lowercase() == "ok" {
            Ok(AcceptStatus::Ok)
        } else {
            // Try to parse a comma-separated list of numbers.
            let codes_result: Result<Vec<u16>, _> =
                input.split(',').map(|s| s.trim().parse::<u16>()).collect();
            match codes_result {
                Ok(codes) => Ok(AcceptStatus::Specific(codes)),
                Err(_) => Err(anyhow::anyhow!("Invalid status codes format")),
            }
        }
    }
    /// Checks if the status is anything other than `None`.
    pub fn is_not_none(&self) -> bool {
        !matches!(self, AcceptStatus::None)
    }
}

/// Main struct for handling the path brute-forcing logic.
pub struct BrutePath {
    /// The target URL template.
    url: String,
    /// The wordlist to use for generating paths.
    wordlist: WordlistType,
    /// The criteria for accepting HTTP status codes.
    accept_status: AcceptStatus,
    /// Flag to determine whether to download found content.
    download: bool,
    /// Flag to run requests in parallel.
    parallel: bool,
    /// The output path for logs or downloaded files.
    out: Option<String>,
}

impl BrutePath {
    /// Creates a new `BrutePath` instance.
    pub fn new(
        url: String,
        wordlist: &str,
        accept_status: &str,
        download: bool,
        parallel: bool,
        out: Option<String>,
    ) -> Self {
        // Parse the wordlist and accept_status strings into their respective enums.
        let wordlist = WordlistType::parse(wordlist).expect("Error parsing wordlist");
        let accept_status = AcceptStatus::parse(accept_status).expect("Error parsing accept status");

        Self {
            url,
            wordlist,
            accept_status,
            download,
            parallel,
            out,
        }
    }

    /// Runs the brute-forcing process in parallel using Tokio tasks.
    pub async fn run_parallel(&self) {
        let wordlists = self.wordlist.get_wordlists();
        let mut threads = Vec::new();

        // Spawn a new task for each word in the wordlist.
        for wordlist in wordlists {
            let url_clone = self.url.clone();
            let accept_status_clone = self.accept_status.clone();
            let is_out_path_set = self.out.is_some();
            let out_path = self.out.clone().unwrap_or_else(|| "./".to_string());
            let download_clone = self.download;

            let t = tokio::spawn(async move {
                let client = reqwest::Client::new();
                let url = url_clone.replace(":path:", &wordlist);
                let res = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Error sending request to {}: {}", url, e);
                        return;
                    }
                };

                // Check if the response status is acceptable.
                let should_log = match &accept_status_clone {
                    AcceptStatus::All => true,
                    AcceptStatus::Ok => res.status().is_success(),
                    AcceptStatus::Specific(codes) => codes.contains(&res.status().as_u16()),
                    AcceptStatus::None => false,
                };

                if should_log {
                    Log::print_found(&url, res.status());
                    if download_clone {
                        // Download the content if the flag is set.
                        let _ = download_file(url, out_path).await;
                    } else if is_out_path_set {
                        // Save the log to a file if an output path is provided.
                        save_log_to_file(&url, res, &out_path).await;
                    }
                }
            });
            threads.push(t);
        }

        // Wait for all tasks to complete.
        for t in threads {
            t.await.unwrap();
        }
    }

    /// Runs the brute-forcing process sequentially.
    pub async fn run_normal(&self) {
        let wordlists = self.wordlist.get_wordlists();
        for wordlist in wordlists {
            let client = reqwest::Client::new();
            let url = self.url.replace(":path:", &wordlist);
            let res = match client.get(&url).send().await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error sending request to {}: {}", url, e);
                    continue; // Continue to the next word on error.
                }
            };
            let is_out_path_set = self.out.is_some();
            let out_path = self.out.clone().unwrap_or_else(|| "./".to_string());

            // Check if the response status is acceptable.
            let should_log = match &self.accept_status {
                AcceptStatus::All => true,
                AcceptStatus::Ok => res.status().is_success(),
                AcceptStatus::Specific(codes) => codes.contains(&res.status().as_u16()),
                AcceptStatus::None => false,
            };

            if should_log {
                Log::print_found(&url, res.status());
                if self.download {
                    // Download the content if the flag is set.
                    let _ = download_file(url, out_path.clone()).await;
                } else if is_out_path_set {
                    // Save the log to a file if an output path is provided.
                    save_log_to_file(&url, res, &out_path).await;
                }
            }
        }
    }

    /// Determines whether to run in parallel or normal mode.
    pub async fn run(&self) {
        if self.parallel {
            self.run_parallel().await;
        } else {
            self.run_normal().await;
        }
    }
}

/// Helper function to save a log entry to a file.
async fn save_log_to_file(url: &String, res: reqwest::Response, out_path: &str) {
    let write_file = WriteFile::new(out_path.to_string());
    // Append the formatted log message to the specified file.
    let _ = write_file
        .append(Log::format(url, res.status()).as_bytes())
        .await;
}
