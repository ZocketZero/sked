//! This module defines constant values used throughout the `sked` application.

/// `BANNER`: A string literal representing the ASCII art banner for the application.
/// This is displayed in the help message.
pub const BANNER: &str = r#"
 ____  _            _ 
/ ___|| | _____  __| |
\___ \| |/ / _ \/ _` |
 ___) |   <  __/ (_| |
|____/|_|\_\___|\__,_|
"#;

/// `BIN_NAME`: The name of the binary executable, used in help messages and for generating completions.
pub const BIN_NAME: &str = "sked";

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TARGET: &str = env!("TARGET");
