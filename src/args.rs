use crate::constant::{BANNER, BIN_NAME};
#[cfg(feature = "brute-path")]
use crate::modules::BrutePathArg;
#[cfg(feature = "public")]
use crate::modules::PubArg;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = BIN_NAME, author, version, about, long_about = None, before_help= BANNER)]
pub struct Argv {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Just say hi!
    Hi,
    /// Add two digit numbers
    #[cfg(feature = "sum")]
    Sum { num1: f64, num2: f64 },
    /// Generate auto complete for any shell.
    #[cfg(feature = "completions")]
    Completions { shell: clap_complete::Shell },
    /// Brute force website's path url.
    #[cfg(feature = "brute-path")]
    BrutePath(BrutePathArg),
    /// Get Public ip
    #[cfg(feature = "public")]
    Pub(PubArg),
}
