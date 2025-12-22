use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use sked::{
    constant::{BANNER, BIN_NAME},
    modules::{BrutePath, PublicIp},
};

#[derive(Parser)]
#[command(name = BIN_NAME, author, version, about, long_about = None, before_help= BANNER)]
struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Just say hi!
    Hi,
    /// Add two digit numbers
    Sum { num1: f64, num2: f64 },
    /// Generate auto complete for any shell.
    Completions { shell: Shell },
    /// Brute force website's path url.
    BrutePath {
        /// Target URL with :path: as placeholder
        #[arg(short, long)]
        url: String,
        /// Wordlist type: range or file (e.g., 1-100 or ./wordlist.txt)
        #[arg(short, long)]
        wordlist: String,
        /// Accepted HTTP status codes (comma separated) (e.g., 200,301) or 'all' or 'ok' for 200-299
        #[arg(short, long, default_value = "ok")]
        accept_status: Option<String>,
        /// Download found files
        #[arg(short, long, default_value_t = false)]
        download: bool,
        /// Run in parallel mode
        #[arg(short, long, default_value_t = false)]
        parallel: bool,
        /// Output file to save results or downloaded files.
        #[arg(short, long, default_value = "./")]
        out: Option<String>,
    },
    /// Get Public ip
    Pub {
        /// get only IPv4
        #[arg(short = '4', long, default_value_t = false)]
        ipv4: bool,
        /// get only IPv6
        #[arg(short = '6', long, default_value_t = false)]
        ipv6: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(command) = args.command {
        match command {
            Command::Pub { ipv4, ipv6 } => PublicIp::new(ipv4, ipv6).run().await?,
            Command::BrutePath {
                url,
                wordlist,
                accept_status,
                download,
                parallel,
                out,
            } => {
                let accept_status = accept_status.unwrap_or_default();
                BrutePath::new(url, &wordlist, &accept_status, download, parallel, out)
                    .run()
                    .await;
            }
            Command::Hi => println!("Hi, have a good day!"),
            Command::Sum { num1, num2 } => println!("{}", num1 + num2),
            Command::Completions { shell } => {
                generate(
                    shell,
                    &mut Args::command(),
                    BIN_NAME,
                    &mut std::io::stdout(),
                );
            }
        }
    } else {
        let _ = Args::command().print_help();
    }
    Ok(())
}
