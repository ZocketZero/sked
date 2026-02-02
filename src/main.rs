use clap::{CommandFactory, Parser};
use sked::args::{Argv, Command};
#[allow(unused)]
use sked::utils::RunCommand;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Argv::parse();
    if let Some(command) = args.command {
        match command {
            #[cfg(feature = "public")]
            Command::Pub(pub_arg) => pub_arg.run().await,

            #[cfg(feature = "brute-path")]
            Command::BrutePath(bp_arg) => bp_arg.run().await,

            Command::Hi => println!("Hi, have a good day!"),

            #[cfg(feature = "sum")]
            Command::Sum { num1, num2 } => println!("{}", num1 + num2),

            #[cfg(feature = "completions")]
            Command::Completions { shell } => {
                use clap_complete::generate;
                use sked::constant::BIN_NAME;
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
