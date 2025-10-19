# Sked

utilities pack for pentest.

## Installation

### Cargo

Install via cargo (You need to install Rust on your device). [official rust document for installation](rust-lang.org/eeetools/install/)

after you have installed rust, run the command bellow to install `Sked`.

```bash
cargo install sked
```

### Basic usage

brute force website's directories via command `brute-path`

```bash
sked brute-path --url http://example.com/:path: --wordlist ./wordlist.txt
```

