use clap::Args;

use crate::{modules::PublicIp, utils::RunCommand};

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

impl RunCommand for PubArg {
    async fn run(&self) {
        let _ = PublicIp::new(self.ipv4, self.ipv6, self.verbose)
            .run()
            .await;
    }
}
