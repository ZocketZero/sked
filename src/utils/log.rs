use reqwest::StatusCode;

use crate::utils::pretty_status::pretty_status;

pub struct Log;

impl Log {
    pub fn print_found(url: &String, status: StatusCode) {
        println!("Found: [{}] {} ", pretty_status(status.as_u16()), url);
    }
    pub fn format(url: &String, status: StatusCode) -> String {
        format!("Found: [{}] {}\n", status.as_u16(), url)
    }
}
