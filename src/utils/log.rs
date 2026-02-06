//! This module provides logging utilities for displaying formatted output.

use reqwest::StatusCode;

use crate::utils::pretty_status::pretty_status;

/// A simple logging utility struct.
pub struct Log;

impl Log {
    /// Prints a formatted "Found" message to the console, including a colorized status code.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL that was found.
    /// * `status` - The HTTP status code of the response.
    pub fn print_found(url: &String, status: StatusCode) {
        println!("Found: [{}] {} ", pretty_status(status.as_u16()), url);
    }

    /// Formats a "Found" message as a string, without printing it.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL that was found.
    /// * `status` - The HTTP status code of the response.
    ///
    /// # Returns
    ///
    /// A formatted string (e.g., "Found: [200] http://example.com\n").
    pub fn format(url: &String, status: StatusCode) -> String {
        format!("Found: [{}] {}\n", status.as_u16(), url)
    }
}
