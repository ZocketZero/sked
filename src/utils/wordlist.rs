//! This module provides utilities for handling wordlists, which can be generated
//! from a numeric range or loaded from a file.

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Represents the source of a wordlist.
enum WordlistType {
    /// A wordlist generated from a continuous range of numbers.
    Range(WordlistRange),
    /// A wordlist loaded from a file path.
    File(WordlistFile),
}

impl WordlistType {
    pub(crate) fn next(&mut self) -> Option<String> {
        match self {
            WordlistType::Range(wordlist_range) => wordlist_range.next(),
            WordlistType::File(f) => f.next(),
        }
    }
}

pub trait WordlistNext {
    fn next(&mut self) -> Option<String>;
}
/// A utility struct for generating wordlists.
pub struct Wordlist {
    wordlist_type: WordlistType,
}

impl Wordlist {
    pub fn next(&mut self) -> Option<String> {
        self.wordlist_type.next()
    }
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let reg = regex::Regex::new(r"^(\d+)-(\d+)$").unwrap();
        if let Some(caps) = reg.captures(input) {
            let start = caps[1]
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid start number"))?;
            let end = caps[2]
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid end number"))?;
            Ok(Self {
                wordlist_type: WordlistType::Range(WordlistRange::new(start, end)),
            })
        } else {
            Ok(Self {
                wordlist_type: WordlistType::File(WordlistFile::new(input)?),
            })
        }
    }
}

struct WordlistRange {
    current: u128,
    max: u128,
}

impl WordlistRange {
    pub fn new(min: u128, max: u128) -> Self {
        Self { current: min, max }
    }
}

impl WordlistNext for WordlistRange {
    fn next(&mut self) -> Option<String> {
        if self.current <= self.max {
            let value = self.current;
            self.current += 1;
            Some(value.to_string())
        } else {
            None
        }
    }
}

struct WordlistFile {
    buffer: BufReader<File>,
}

impl WordlistFile {
    fn new(path: &str) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            buffer: BufReader::new(file),
        })
    }
}

impl WordlistNext for WordlistFile {
    fn next(&mut self) -> Option<String> {
        let mut line = String::new();
        if let Ok(c) = self.buffer.read_line(&mut line)
            && c > 0
        {
            Some(line.trim_end().to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_wordlist_range() {
        let mut wl = Wordlist::parse("10-20").unwrap();
        let mut count = 10;
        while let Some(n) = wl.next() {
            assert_eq!(n, count.to_string());
            count += 1;
        }
    }

    #[test]
    fn wordlist_from_file() {
        // Create a dummy file for testing
        let content = "line1\nline2\nline3";
        fs::write("test_wordlist.txt", content).unwrap();

        let mut wordlist = Wordlist::parse("test_wordlist.txt").unwrap();
        let check = ["line1", "line2", "line3"];
        for i in 0..3 {
            if let Some(wl) = wordlist.next() {
                assert_eq!(check[i], wl.as_str());
            }
        }

        // Clean up the dummy file
        fs::remove_file("test_wordlist.txt").unwrap();
    }
}
