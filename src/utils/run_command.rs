//! This module defines a generic trait for running commands.
//! This is used to create a common interface for all runnable subcommands.

use std::future::Future;

/// A trait for structs that can be executed as a command.
pub trait RunCommand {
    /// The main execution function for the command.
    /// It returns a future that resolves when the command is complete.
    fn run(&self) -> impl Future<Output = ()>;
}
