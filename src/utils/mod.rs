// Declares private utility modules. These modules contain functions and structures
// that provide various functionalities used across the `sked` application.
mod download_file;    // Handles downloading files from URLs.
mod log;              // Provides logging functionalities for application events.
mod pretty_status;    // Manages and displays pretty status messages during operations.
mod run_command;      // Utility for running external shell commands.
mod wordlist;         // Manages wordlist operations, such as loading and type handling.
mod write_file;       // Handles writing content to files.

// Re-exports public items from the utility modules. These items are made public
// so they can be easily accessed and used by other parts of the `sked` crate.
pub use download_file::download_file; // Function to download files.
pub use log::Log;                     // Trait or struct for logging.
pub use pretty_status::PrettyStatus;  // Struct to handle and display operation status.
pub use run_command::RunCommand;      // Struct to execute and manage external commands.
pub use wordlist::Wordlist;           // Struct representing a loaded wordlist.
pub use wordlist::WordlistType;       // Enum for different types of wordlists.
pub use write_file::WriteFile;        // Struct to handle writing data to files.