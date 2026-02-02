use clap::Parser;

fn main() {
    btc_wallet::utils::Args::parse().run();
}
