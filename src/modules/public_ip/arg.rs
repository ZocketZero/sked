//! This module defines the command-line arguments for the `pub` subcommand,
//! which is used to fetch the user's public IP address.

use crate::{modules::PublicIp, utils::RunCommand};
use clap::Args;

/// Arguments for the `pub` command.
#[derive(Args)]
pub struct PubArg {
    /// If set, only the IPv4 address will be fetched and displayed.
    #[arg(short = '4', long, default_value_t = false)]
    pub ipv4: bool,
    /// If set, only the IPv6 address will be fetched and displayed.
    #[arg(short = '6', long, default_value_t = false)]
    pub ipv6: bool,
    /// If set, provides a more detailed, verbose output (e.g., JSON).
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

impl RunCommand for PubArg {
    /// Executes the public IP fetching logic based on the provided arguments.
    async fn run(&self) {
        // Create a new `PublicIp` instance and run it.
        let _ = PublicIp::new(self.ipv4, self.ipv6, self.verbose)
            .run()
            .await;
    }
}
