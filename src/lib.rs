//! This is the main library crate for the `sked` application.
//! It organizes the different modules that make up the application's functionality.

/// `args`: Defines the command-line arguments and subcommands using `clap`.
pub mod args;
/// `constant`: Contains constant values used throughout the application, such as the banner.
pub mod constant;
/// `modules`: Contains the core logic for each of the application's features (subcommands).
pub mod modules;
/// `utils`: Provides common utility functions and traits used by various modules.
pub mod utils;
