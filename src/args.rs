//! This module defines the command-line arguments for the `sked` application.
//! It uses the `clap` crate to parse and manage arguments.

use crate::constant::{BANNER, BIN_NAME};
#[cfg(feature = "brute-path")]
use crate::modules::BrutePathArg;
#[cfg(feature = "public")]
use crate::modules::PubArg;
use clap::{Parser, Subcommand};

/// Represents the main structure for command-line arguments.
/// It includes the command to be executed.
#[derive(Parser)]
#[command(name = BIN_NAME, author, version, about, long_about = None, before_help= BANNER)]
pub struct Argv {
    /// The command to execute.
    #[clap(subcommand)]
    pub command: Option<Command>,
}

/// Enumerates the available subcommands for the `sked` application.
/// Each variant corresponds to a specific feature, enabled via feature flags.
#[derive(Subcommand)]
pub enum Command {
    /// A simple command that prints a greeting message.
    Hi,
    /// A command to add two floating-point numbers.
    #[cfg(feature = "sum")]
    Sum { num1: f64, num2: f64 },
    /// A command to generate shell completions.
    #[cfg(feature = "completions")]
    Completions {
        /// The shell for which to generate completions.
        shell: clap_complete::Shell,
    },
    /// A command to brute-force website paths.
    /// This command takes arguments defined in `BrutePathArg`.
    #[cfg(feature = "brute-path")]
    BrutePath(BrutePathArg),
    /// A command to get the public IP address.
    /// This command takes arguments defined in `PubArg`.
    #[cfg(feature = "public")]
    Pub(PubArg),
    /// A command to generate a Bitcoin wallet.
    /// This command uses arguments from the `btc_wallet` crate.
    #[cfg(feature = "bitcoin")]
    Btc(btc_wallet::utils::Args),
}
