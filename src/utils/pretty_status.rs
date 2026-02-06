//! This module provides functions and a trait for pretty-printing HTTP status codes with colors.

use colored::Colorize;
use reqwest::StatusCode;

/// Takes a status code and returns a colorized string representation.
///
/// - 2xx are green.
/// - 3xx are yellow.
/// - 4xx are red.
/// - 5xx are magenta.
/// - Others are in the default color.
pub fn pretty_status(status: u16) -> String {
    match status {
        200..=299 => format!("{}", status).green().to_string(),
        300..=399 => format!("{}", status).yellow().to_string(),
        400..=499 => format!("{}", status).red().to_string(),
        500..=599 => format!("{}", status).magenta().to_string(),
        _ => format!("{}", status).normal().to_string(),
    }
}

/// A trait to allow types like `u16` and `StatusCode` to be pretty-printed directly.
pub trait PrettyStatus {
    /// Returns a pretty-printed, colorized string of the status.
    fn pretty_status(&self) -> String;
}

/// Implementation of `PrettyStatus` for `u16`.
impl PrettyStatus for u16 {
    fn pretty_status(&self) -> String {
        pretty_status(*self)
    }
}

/// Implementation of `PrettyStatus` for `reqwest::StatusCode`.
impl PrettyStatus for StatusCode {
    fn pretty_status(&self) -> String {
        pretty_status(self.as_u16())
    }
}
