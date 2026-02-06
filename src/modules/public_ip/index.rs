//! This module contains the core logic for the `public-ip` command.
//! It defines the `PublicIp` struct and the methods to fetch and display the IP address.

use crate::modules::public_ip::{
    config,
    request::{self, IPtype},
};

/// The main struct for handling the public IP fetching logic.
pub struct PublicIp {
    /// Determines whether to fetch IPv4, IPv6, or both.
    ip_select: IpSelect,
    /// If true, a more verbose output is printed.
    verbose: bool,
}

/// Enum to represent the user's IP selection.
#[derive(PartialEq)]
enum IpSelect {
    /// Fetch both IPv4 and IPv6 addresses.
    All,
    /// Fetch only the IPv4 address.
    IPv4,
    /// Fetch only the IPv6 address.
    IPv6,
}

impl IpSelect {
    /// Creates a new `IpSelect` variant based on boolean flags.
    pub fn new(ipv4: bool, ipv6: bool) -> Self {
        if ipv4 {
            Self::IPv4
        } else if ipv6 {
            Self::IPv6
        } else {
            Self::All
        }
    }
}

impl PublicIp {
    /// Creates a new `PublicIp` instance.
    pub fn new(ipv4: bool, ipv6: bool, verbose: bool) -> Self {
        Self {
            ip_select: IpSelect::new(ipv4, ipv6),
            verbose,
        }
    }

    /// The main entry point for running the IP fetching logic.
    pub async fn run(&self) -> anyhow::Result<()> {
        self.print_ip().await?;
        Ok(())
    }

    /// Fetches and prints detailed IP information from a different service.
    /// Note: This function appears to be unused in the main execution flow.
    pub async fn info() -> anyhow::Result<()> {
        let client = request::client(IPtype::IPv4)?;
        let ipinfo = match client.get("https://ip-api.com").send().await {
            Ok(res) => res.text().await?,
            Err(_) => "None".to_string(),
        };
        println!("{}", ipinfo);
        Ok(())
    }

    /// Fetches and prints the IP address(es) based on the user's selection.
    async fn print_ip(&self) -> anyhow::Result<()> {
        // Select the appropriate URL based on the verbosity setting.
        let ipv4_url = if self.verbose {
            config::URL_IPV4_V
        } else {
            config::URL_IPV4
        };
        let ipv6_url = if self.verbose {
            config::URL_IPV6_V
        } else {
            config::URL_IPV6
        };

        // Fetch and print IPv4 if requested.
        if self.ip_select == IpSelect::All || self.ip_select == IpSelect::IPv4 {
            let client = request::client(IPtype::IPv4)?;
            let ipv4 = match client.get(ipv4_url).send().await {
                Ok(res) => res.text().await?,
                Err(err) => err.to_string(),
            };
            println!("ipv4: {}", ipv4);
        }

        // Fetch and print IPv6 if requested.
        if self.ip_select == IpSelect::IPv6 || self.ip_select == IpSelect::All {
            let client = request::client(IPtype::IPv6)?;
            let ipv6 = match client.get(ipv6_url).send().await {
                Ok(res) => res.text().await?,
                Err(err) => err.to_string(),
            };
            println!("ipv6: {}", ipv6);
        }
        Ok(())
    }
}
