//! This module provides utilities for handling wordlists, which can be generated
//! from a numeric range or loaded from a file.

use std::fs;
use anyhow::Ok;

/// Represents the source of a wordlist.
pub enum WordlistType {
    /// A wordlist generated from a continuous range of numbers.
    Range(u32, u32),
    /// A wordlist loaded from a file path.
    File(String),
}

impl WordlistType {
    /// Parses an input string to determine if it's a range (e.g., "1-100") or a file path.
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let reg = regex::Regex::new(r"^(\d+)-(\d+)$").unwrap();
        if let Some(caps) = reg.captures(input) {
            let start = caps[1].parse().map_err(|_| anyhow::anyhow!("Invalid start number"))?;
            let end = caps[2].parse().map_err(|_| anyhow::anyhow!("Invalid end number"))?;
            Ok(Self::Range(start, end))
        } else {
            Ok(Self::File(input.to_string()))
        }
    }

    /// Retrieves the wordlist as a vector of strings, based on the enum variant.
    pub fn get_wordlists(&self) -> Vec<String> {
        match self {
            WordlistType::Range(b, e) => Wordlist::range(*b, *e),
            WordlistType::File(path) => Wordlist::from_file(path.clone()),
        }
    }
}

/// A utility struct for generating wordlists.
pub struct Wordlist;

impl Wordlist {
    /// Creates a vector of strings representing a numeric range.
    pub fn range(from: u32, to: u32) -> Vec<String> {
        (from..=to).map(|i| i.to_string()).collect()
    }

    /// Reads a file and returns its lines as a vector of strings.
    pub fn from_file(path: String) -> Vec<String> {
        fs::read_to_string(path)
            .expect("Failed to read wordlists file")
            .lines()
            .map(String::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wordlist_range() {
        let wl = WordlistType::parse("10-20").unwrap();
        if let WordlistType::Range(start, end) = wl {
            assert_eq!(start, 10);
            assert_eq!(end, 20);
        } else {
            panic!("Failed to parse wordlist range");
        }
    }

    #[test]
    fn wordlist_from_file() {
        // Create a dummy file for testing
        let content = "line1\nline2\nline3";
        fs::write("test_wordlist.txt", content).unwrap();

        let r = Wordlist::from_file("test_wordlist.txt".to_string());
        assert_eq!(r, vec!["line1", "line2", "line3"]);

        // Clean up the dummy file
        fs::remove_file("test_wordlist.txt").unwrap();
    }

    #[test]
    fn wordlist_range() {
        let wordlist = Wordlist::range(0, 5);
        assert_eq!(wordlist, vec!["0", "1", "2", "3", "4", "5"]);
    }
}
