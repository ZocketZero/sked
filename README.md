# Sked: A Pentester's Utility Belt

`sked` is a command-line toolkit written in Rust, designed to streamline common penetration testing tasks. It's a collection of utilities that range from web reconnaissance to cryptocurrency wallet management.

## Features

*   **Path Brute-forcing**: Discover hidden directories and files on a web server.
*   **Public IP Discovery**: Quickly find your public IPv4 and IPv6 addresses.
*   **Bitcoin Wallet Generation**: Create new Bitcoin wallets.
*   **And more**: Includes other handy tools.

## Installation

### From Crates.io (Recommended)

If you have the Rust toolchain installed, you can easily install `sked` from [crates.io](https://crates.io/crates/sked).

```bash
cargo install sked
```

### Build from Source

You can also build `sked` from the source code.

1.  Clone the repository:
    ```bash
    git clone https://github.com/ZocketZero/sked.git
    cd sked
    ```

2.  Build the project:
    ```bash
    cargo build --release
    ```
    The binary will be located at `target/release/sked`.

## Usage

Here are some examples of how to use `sked`.

### Brute-force Web Directories

Use the `brute-path` command to find valid paths on a web server.

```bash
sked brute-path --url http://example.com/:path: --wordlist ./path/to/wordlist.txt
```

*   `--url`: The target URL. Use `:path:` as a placeholder for the wordlist entries.
*   `--wordlist`: The path to the wordlist file.

### Check Public IP

To find your public IP address, use the `pub` command.

```bash
sked pub
```

### Generate a Bitcoin Wallet

Create a new Bitcoin wallet with the `btc` command.

```bash
sked btc
```

### View All Commands

For a full list of commands and their options, use the `--help` flag.

```bash
sked --help
```

## Development

### Running Tests

To run the test suite, use the following command:

```bash
cargo test
```