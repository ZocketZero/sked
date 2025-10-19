use std::fs;

use anyhow::Ok;

pub enum WordlistType {
    Range(u32, u32),
    File(String),
}

impl WordlistType {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let reg = regex::Regex::new(r"^(\d+)-(\d+)$").unwrap();
        if reg.is_match(input) {
            let ca = input.split("-").collect::<Vec<&str>>();
            if ca.len() != 2 {
                return Err(anyhow::anyhow!("Invalid wordlist range format"));
            }
            let start: u32 = ca[0]
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid start number"))?;
            let end: u32 = ca[1]
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid end number"))?;
            Ok(Self::Range(start, end))
        } else {
            Ok(Self::File(input.to_string()))
        }
    }
    pub fn get_wordlists(&self) -> Vec<String> {
        match self {
            WordlistType::Range(b, e) => Wordlist::range(*b, *e),
            WordlistType::File(path) => Wordlist::file(path.clone()),
        }
    }
}

/// Manage about Wordlist.
pub struct Wordlist;

impl Wordlist {
    /// Create wordlists from range number
    pub fn range(from: u32, to: u32) -> Vec<String> {
        let mut wordlists: Vec<String> = Vec::new();

        for i in from..to + 1 {
            wordlists.push(i.to_string());
        }

        wordlists
    }

    /// Get wordlists from file.
    pub fn file(path: String) -> Vec<String> {
        let read_string = fs::read_to_string(path).expect("Failed to read wordlists file");
        let content: Vec<&str> = read_string.split("\n").collect();

        content.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wordlist_range() {
        let wl = WordlistType::parse("10-20");
        match wl {
            std::result::Result::Ok(WordlistType::Range(start, end)) => {
                assert_eq!(start, 10);
                assert_eq!(end, 20);
            }
            _ => panic!("Failed to parse wordlist range"),
        }
    }

    #[test]
    fn wordlist_file() {
        let r = Wordlist::file("./Cargo.toml".to_string());
        let content = r.join("\n");
        let cargo_toml = fs::read_to_string("./Cargo.toml").expect("Failed to read wordlists file");
        assert_eq!(content, cargo_toml);
    }

    #[test]
    fn wordlist_range() {
        let wordlist = Wordlist::range(0, 20);
        assert_eq!(
            wordlist,
            vec![
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20"
            ]
        )
    }
}
