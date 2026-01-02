use clap::Args;

#[derive(Args)]
pub struct PubArg {
    /// get only IPv4
    #[arg(short = '4', long, default_value_t = false)]
    pub ipv4: bool,
    /// get only IPv6
    #[arg(short = '6', long, default_value_t = false)]
    pub ipv6: bool,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
