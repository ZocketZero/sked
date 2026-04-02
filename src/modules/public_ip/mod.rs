//! This module groups all the components related to the `public-ip` feature.
//! It declares and re-exports the necessary structs and arguments for external use.

// Private modules for internal organization.
mod arg; // Defines command-line arguments for the `pub` command.
mod config; // Contains configuration constants like API URLs.
mod index; // The core logic for fetching and displaying the public IP.
mod request; // Handles the underlying HTTP requests.

// Re-export the public-facing components.
pub use arg::PubArg; // The arguments struct for `clap`.
pub use index::PublicIp; // The main struct that implements the feature's logic.
