use crate::modules::public_ip::{
    config,
    request::{self, IPtype},
};

pub struct PublicIp {
    ip_select: IpSelect,
    verbose: bool,
}

#[derive(PartialEq)]
enum IpSelect {
    All,
    IPv4,
    IPv6,
}

impl IpSelect {
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
    pub fn new(ipv4: bool, ipv6: bool, verbose: bool) -> Self {
        Self {
            ip_select: IpSelect::new(ipv4, ipv6),
            verbose,
        }
    }
    pub async fn run(&self) -> anyhow::Result<()> {
        self.print_ip().await?;
        Ok(())
    }

    pub async fn info() -> anyhow::Result<()> {
        let client = request::client(IPtype::IPv4)?;
        let ipinfo = match client.get("https://ip-api.com").send().await {
            Ok(res) => res.text().await?,
            Err(_) => "None".to_string(),
        };
        println!("{}", ipinfo);
        Ok(())
    }

    async fn print_ip(&self) -> anyhow::Result<()> {
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
        if self.ip_select == IpSelect::All || self.ip_select == IpSelect::IPv4 {
            let client = request::client(IPtype::IPv4)?;
            let ipv4 = match client.get(ipv4_url).send().await {
                Ok(res) => res.text().await?,
                Err(err) => err.to_string(),
            };
            println!("ipv4: {}", ipv4);
        }

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
