use reqwest::StatusCode;

use crate::utils::pretty_status::pretty_status;

pub struct Log;

impl Log {
   pub fn print_found(url: &String, status: StatusCode) {
       println!("Found: [{}] {} ", pretty_status(status.as_u16()), url);
   } 
}