use colored::Colorize;
use reqwest::StatusCode;

pub fn pretty_status(status: u16) -> String {
    match status {
        200..=299 => format!("{}", status).green().to_string(),
        300..=399 => format!("{}", status).yellow().to_string(),
        400..=499 => format!("{}", status).red().to_string(),
        500..=599 => format!("{}", status).magenta().to_string(),
        _ => format!("{}", status).normal().to_string(),
    }
}

pub trait PrettyStatus {
     fn pretty_status(&self) -> String; 
}

impl PrettyStatus for u16 {
    fn pretty_status(&self) -> String {
        pretty_status(*self)
    }
}

impl PrettyStatus for StatusCode {
    fn pretty_status(&self) -> String {
        pretty_status(self.as_u16())
    }
}