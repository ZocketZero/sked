# Sked

Utilities pack for pentest.

## Installation

### Cargo

Install via cargo (You need to install Rust on your device). [official rust document for installation](rust-lang.org/eeetools/install/)

After you have installed rust, run the command bellow to install `Sked`.

```bash
cargo install sked
```

### Basic usage

Brute force website's directories via command `brute-path`

```bash
sked brute-path --url http://example.com/:path: --wordlist ./wordlist.txt
```

Check your public IP.

```bash
sked pub
```
