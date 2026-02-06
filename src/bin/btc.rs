//! This binary is a wrapper for the `btc_wallet` crate's command-line utility.
//! It allows `sked` to expose Bitcoin wallet generation as a subcommand.

use clap::Parser;

/// The main entry point for the `btc` binary.
/// It parses command-line arguments using `btc_wallet::utils::Args` and runs the command.
fn main() {
    btc_wallet::utils::Args::parse().run();
}
