# Gemini Code Understanding

This document provides a comprehensive overview of the `sked` project, a command-line utility for penetration testing written in Rust.

## Project Overview

`sked` is a collection of tools designed for penetration testers. It is built in Rust and distributed as a command-line application. The main features include:

*   **Path Brute-forcing:** A tool to brute-force website directories using a wordlist.
*   **Public IP Discovery:** A utility to fetch and display the user's public IPv4 and IPv6 addresses.
*   **Bitcoin Wallet Generation:** A feature for creating Bitcoin wallets.
*   **Other Utilities:** Includes simple tools like a calculator (`sum`) and a greeting (`Hi`).

The project is structured into several modules, with each command corresponding to a specific feature. It uses `clap` for command-line argument parsing and `tokio` for asynchronous operations. The project is configured for continuous integration and deployment using GitHub Actions.

## Building and Running

### Building the Project

To build the project from source, you will need to have the Rust toolchain installed.

1.  Clone the repository:
    ```bash
    git clone https://github.com/ZocketZero/sked.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd sked
    ```
3.  Build the project:
    ```bash
    cargo build
    ```

### Running the Application

Once built, you can run the application using `cargo run`:

```bash
cargo run -- <COMMAND>
```

For example, to see the help message:

```bash
cargo run -- --help
```

To run a specific command, like checking the public IP:

```bash
cargo run -- pub
```

### Running Tests

The project has a suite of tests that can be run using the following command:

```bash
cargo test
```

## Development Conventions

*   **Asynchronous Operations:** The project uses `tokio` for asynchronous operations, particularly for network requests in the `brute-path` and `public-ip` modules.
*   **Modular Structure:** The codebase is organized into modules, with each feature residing in its own module under `src/modules`. This promotes separation of concerns and code organization.
*   **Feature Flags:** The project uses feature flags to enable or disable specific commands during compilation. This allows for creating different builds of the application with varying feature sets.
*   **Error Handling:** The project uses the `anyhow` crate for flexible and easy error handling.
*   **CI/CD:** The project is configured with GitHub Actions for continuous integration and deployment. The CI pipeline runs tests on every push and pull request, while the CD pipeline publishes new releases to crates.io and creates GitHub releases with pre-compiled binaries for various platforms.
