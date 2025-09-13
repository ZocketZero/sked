use std::fs;

pub struct Wordlist;

impl Wordlist {
    pub fn range(from: u32, to: u32) -> Vec<String> {
        let mut wordlists: Vec<String> = Vec::new();

        for i in from..to + 1 {
            wordlists.push(i.to_string());
        }

        wordlists
    }

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
    fn wordlist_file() {
        let r = Wordlist::file("./Cargo.toml".to_string());
        for msg in r {
            println!("{}", msg);
        }
    }

    #[test]
    fn wordlist_range() {
        let wordlist = Wordlist::range(0, 20);
        println!("{:?}", wordlist);
        assert_eq!(0, 0)
    }
}
