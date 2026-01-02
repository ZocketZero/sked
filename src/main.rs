use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use sked::{
    constant::{BANNER, BIN_NAME},
    modules::{BrutePath, BrutePathArg, PubArg, PublicIp},
};

#[derive(Parser)]
#[command(name = BIN_NAME, author, version, about, long_about = None, before_help= BANNER)]
struct Argv {
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
    BrutePath(BrutePathArg),
    /// Get Public ip
    Pub(PubArg),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Argv::parse();
    if let Some(command) = args.command {
        match command {
            Command::Pub(pub_arg) => {
                PublicIp::new(pub_arg.ipv4, pub_arg.ipv6, pub_arg.verbose)
                    .run()
                    .await?;
            }
            Command::BrutePath(bp_arg) => {
                let accept_status = bp_arg.accept_status.unwrap_or_default();
                BrutePath::new(
                    bp_arg.url,
                    &bp_arg.wordlist,
                    &accept_status,
                    bp_arg.download,
                    bp_arg.parallel,
                    bp_arg.out,
                )
                .run()
                .await;
            }
            Command::Hi => println!("Hi, have a good day!"),
            Command::Sum { num1, num2 } => println!("{}", num1 + num2),
            Command::Completions { shell } => {
                generate(
                    shell,
                    &mut Argv::command(),
                    BIN_NAME,
                    &mut std::io::stdout(),
                );
            }
        }
    } else {
        let _ = Argv::command().print_help();
    }
    Ok(())
}
