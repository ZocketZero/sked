use crate::utils::{Log, WordlistType, WriteFile, download_file};

#[derive(Default, Clone)]
/// ## Accepted http status codes.
pub enum AcceptStatus {
    /// ## Accept all status codes.
    All,
    /// ## Accept specific status codes.
    Specific(Vec<u16>),
    #[default]
    /// Accept status within 200-299 range.
    Ok,
    /// ## Not accept any status codes.
    None,
}

impl AcceptStatus {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        if input.is_empty() {
            Ok(AcceptStatus::None)
        } else if input.to_lowercase() == "all" || input == "-" {
            Ok(AcceptStatus::All)
        } else if input.to_lowercase() == "ok" {
            Ok(AcceptStatus::Ok)
        } else {
            let codes_result: Result<Vec<u16>, _> =
                input.split(',').map(|s| s.trim().parse::<u16>()).collect();
            match codes_result {
                Ok(codes) => Ok(AcceptStatus::Specific(codes)),
                Err(_) => Err(anyhow::anyhow!("Invalid status codes format")),
            }
        }
    }
    pub fn is_not_none(&self) -> bool {
        !matches!(self, AcceptStatus::None)
    }
}

/// ## Brute force website's path url.
pub struct BrutePath {
    /// ## Target url.
    url: String,
    /// Wordlist type. (Range, File)
    wordlist: WordlistType,
    /// Accepted http status codes.
    accept_status: AcceptStatus,
    /// Download found files.
    download: bool,
    /// Run in parallel mode.
    parallel: bool,
    /// Output file to save results or downloaded files.
    out: Option<String>,
}

impl BrutePath {
    pub fn new(
        url: String,
        wordlist: &str,
        accept_status: &str,
        download: bool,
        parallel: bool,
        out: Option<String>,
    ) -> Self {
        let wordlist = match WordlistType::parse(wordlist) {
            Ok(wl) => wl,
            Err(e) => {
                panic!("Error parsing wordlist: {}", e);
            }
        };
        let accept_status = match AcceptStatus::parse(accept_status) {
            Ok(as_) => as_,
            Err(e) => {
                panic!("Error parsing accept status: {}", e);
            }
        };
        Self {
            url,
            wordlist,
            accept_status,
            download,
            parallel,
            out,
        }
    }

    pub async fn run_parallel(&self) {
        let wordlists = self.wordlist.get_wordlists();
        let mut threads = Vec::new();
        for wordlist in wordlists {
            let url_clone = self.url.clone();
            let accept_status_clone = self.accept_status.clone();
            let is_out_path_set = self.out.is_some();
            let out_path = match self.out.clone() {
                Some(as_) => as_,
                None => "./".to_string(),
            };
            let download_clone = self.download;

            let t = tokio::spawn(async move {
                let client = reqwest::Client::new();
                let url = url_clone.replace(":path:", &wordlist);
                let res = client.get(&url).send().await;

                let res = match res {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Error sending request to {}: {}", url, e);
                        return;
                    }
                };

                if accept_status_clone.is_not_none() {
                    match &accept_status_clone {
                        AcceptStatus::All => {
                            Log::print_found(&url, res.status());
                            // check --download flag
                            if download_clone {
                                // download web page or contents to out_path
                                let _ = download_file(url, out_path).await;
                            } else if is_out_path_set {
                                // if --out is set without --download, save results to file
                                save_log_to_file(&url, res, &out_path).await;
                            }
                        }
                        AcceptStatus::Ok => {
                            if res.status().is_success() {
                                Log::print_found(&url, res.status());
                                // download web page or contents to out_path
                                if download_clone {
                                    // download web page or contents to out_path
                                    let _ = download_file(url, out_path).await;
                                } else if is_out_path_set {
                                    // if --out is set without --download, save results to file
                                    save_log_to_file(&url, res, &out_path).await;
                                }
                            }
                        }
                        AcceptStatus::Specific(codes) => {
                            if codes.contains(&res.status().as_u16()) {
                                Log::print_found(&url, res.status());
                                if download_clone {
                                    let _ = download_file(url, out_path).await;
                                } else if is_out_path_set {
                                    save_log_to_file(&url, res, &out_path).await;
                                }
                            }
                        }
                        AcceptStatus::None => {}
                    }
                } else {
                    Log::print_found(&url, res.status());
                    if download_clone {
                        let _ = download_file(url, out_path).await;
                    }
                }
            });
            threads.push(t);
        }
        for t in threads {
            t.await.unwrap();
        }
    }
    pub async fn run_normal(&self) {
        let wordlists = self.wordlist.get_wordlists();
        for wordlist in wordlists {
            let client = reqwest::Client::new();
            let url = self.url.replace(":path:", &wordlist);
            let res = client.get(&url).send().await;
            let is_out_path_set = self.out.is_some();
            let out_path = match self.out.clone() {
                Some(as_) => as_,
                None => "./".to_string(),
            };

            let res = match res {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error sending request to {}: {}", url, e);
                    return;
                }
            };

            if self.accept_status.is_not_none() {
                match &self.accept_status {
                    AcceptStatus::All => {
                        Log::print_found(&url, res.status());
                        if self.download {
                            let _ = download_file(url, out_path.clone()).await;
                        } else if is_out_path_set {
                            save_log_to_file(&url, res, &out_path).await;
                        }
                    }
                    AcceptStatus::Ok => {
                        if res.status().is_success() {
                            Log::print_found(&url, res.status());
                            if self.download {
                                let _ = download_file(url, out_path).await;
                            } else if is_out_path_set {
                                save_log_to_file(&url, res, &out_path).await;
                            }
                        }
                    }
                    AcceptStatus::Specific(codes) => {
                        if codes.contains(&res.status().as_u16()) {
                            Log::print_found(&url, res.status());
                            if self.download {
                                let _ = download_file(url, out_path).await;
                            } else if is_out_path_set {
                                save_log_to_file(&url, res, &out_path).await;
                            }
                        }
                    }
                    AcceptStatus::None => {}
                }
            } else {
                Log::print_found(&url, res.status());
            }
        }
    }
    pub async fn run(&self) {
        if self.parallel {
            self.run_parallel().await;
        } else {
            self.run_normal().await;
        }
    }
}

async fn save_log_to_file(url: &String, res: reqwest::Response, out_path: &str) {
    let write_file = WriteFile::new(out_path.to_string());
    let _ = write_file
        .append(Log::format(url, res.status()).as_bytes().to_vec().as_ref())
        .await;
}
