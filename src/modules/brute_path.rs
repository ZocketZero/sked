use crate::utils::WordlistType;

pub enum AcceptStatus {
    All,
    Specific(Vec<u16>),
}

/// Brute force website's path url.
pub struct BrutePath {
    url: String,
    wordlist: WordlistType,
    accept_status: AcceptStatus,
    download: bool,
}

impl BrutePath {
    pub fn new(
        url: String,
        wordlist: WordlistType,
        accept_status: AcceptStatus,
        download: bool,
    ) -> Self {
        Self {
            url,
            wordlist,
            accept_status,
            download,
        }
    }
    pub async fn run(&self) {
        let client = reqwest::Client::new();
    }
}
