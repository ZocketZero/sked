//! The main entry point for the `sked` command-line application.
//! This module parses command-line arguments and executes the corresponding command.

use clap::{CommandFactory, Parser};
use sked::args::{Argv, Command};
#[allow(unused)]
use sked::utils::RunCommand;

/// The main asynchronous function that runs the application.
/// It parses arguments and dispatches to the appropriate command handler.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command-line arguments into the Argv struct.
    let args = Argv::parse();

    // Check if a subcommand was provided.
    if let Some(command) = args.command {
        // Match the subcommand and execute its logic.
        match command {
            // Handles the 'pub' command for fetching the public IP.
            #[cfg(feature = "public")]
            Command::Pub(pub_arg) => pub_arg.run().await,

            // Handles the 'brute-path' command for directory brute-forcing.
            #[cfg(feature = "brute-path")]
            Command::BrutePath(bp_arg) => bp_arg.run().await,

            // Handles the 'btc' command for Bitcoin wallet generation.
            #[cfg(feature = "bitcoin")]
            Command::Btc(btc_args) => {
                btc_args.run();
            }

            // Handles the 'Hi' command to print a simple greeting.
            Command::Hi => println!("Hi, have a good day!"),

            // Handles the 'sum' command for adding two numbers.
            #[cfg(feature = "sum")]
            Command::Sum { num1, num2 } => println!("{}", num1 + num2),

            // Handles the 'completions' command to generate shell completions.
            #[cfg(feature = "completions")]
            Command::Completions { shell } => {
                shell.generate(Argv::command());
            }
        }
    } else {
        // If no subcommand is provided, print the help message.
        let _ = Argv::command().print_help();
    }
    // Return Ok to indicate successful execution.
    Ok(())
}
